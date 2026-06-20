use crate::pipeline::context::PipelineContext;
use crate::traits::stage::EngineStage;

pub struct PipelineRunner {
    stages: Vec<Box<dyn EngineStage>>,
}

impl PipelineRunner {
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
        }
    }

    pub fn with_stages(stages: Vec<Box<dyn EngineStage>>) -> Self {
        Self { stages }
    }

    pub fn add_stage(
        &mut self,
        stage: Box<dyn EngineStage>,
    ) {
        self.stages.push(stage);
    }

    pub fn run(
        &self,
        data: Vec<u8>,
        lanes: usize,
    ) -> Vec<u8> {
        let lanes = lanes.max(1);

        if lanes == 1 {
            let mut ctx = PipelineContext::default();
            let mut current_data = data;
            for stage in &self.stages {
                current_data = stage.execute(current_data, &mut ctx);
            }
            current_data
        } else {
            use rayon::prelude::*;

            // Run L independent lanes in parallel.
            // Each lane gets a distinct input by appending the lane index to the data.
            // This ensures each lane generates a completely different graph and chaos stream.
            let lane_outputs: Vec<Vec<u8>> = (0..lanes).into_par_iter().map(|lane_idx| {
                let mut ctx = PipelineContext::default();
                let mut current_data = data.clone();
                
                // Add lane index to differentiate inputs
                current_data.extend_from_slice(&(lane_idx as u64).to_le_bytes());

                for stage in &self.stages {
                    current_data = stage.execute(current_data, &mut ctx);
                }
                current_data
            }).collect();

            // Concatenate all lane outputs together
            let mut combined = Vec::with_capacity(lane_outputs.iter().map(|v| v.len()).sum());
            for output in lane_outputs {
                combined.extend(output);
            }
            
            // Re-hash the combined output into a 64-byte or 128-byte block?
            // Actually, XaorEngine has a FinalizerStage at the end, so the lane_outputs
            // ALREADY went through the FinalizerStage and are 64/128 bytes each.
            // So we return the concatenated `L * 64` bytes. The Engine will hash it.
            combined
        }
    }
}

impl Default for PipelineRunner {
    fn default() -> Self {
        Self::new()
    }
}
