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
        mut data: Vec<u8>,
    ) -> Vec<u8> {
        let mut ctx = PipelineContext::default();

        for stage in &self.stages {
            println!("Running stage: {}", stage.name());
            data = stage.execute(data, &mut ctx);
        }

        data
    }
}
