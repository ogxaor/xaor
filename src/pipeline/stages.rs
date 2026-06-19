use crate::compound::CompoundEngine;
use crate::entropy::EntropyVector;
use crate::finalizer::FinalizerEngine;
use crate::memory::MemoryArena;
use crate::recycler::RecyclerEngine;
use crate::seed::SeedEngine;
use crate::topology::TopologyEngine;
use crate::traits::stage::EngineStage;

use super::context::PipelineContext;

pub struct SeedStage {
    entropy: EntropyVector,
}

impl SeedStage {
    pub fn new(entropy: EntropyVector) -> Self {
        Self { entropy }
    }
}

impl EngineStage for SeedStage {
    fn name(&self) -> &'static str {
        "SeedStage"
    }

    fn execute(&self, input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        let seed_engine = SeedEngine::new();
        seed_engine
            .generate(&input, &self.entropy)
            .expect("seed generation is infallible")
            .root
            .to_vec()
    }
}

pub struct TopologyStage {
    node_count: usize,
}

impl TopologyStage {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl EngineStage for TopologyStage {
    fn name(&self) -> &'static str {
        "TopologyStage"
    }

    fn execute(&self, input: Vec<u8>, ctx: &mut PipelineContext) -> Vec<u8> {
        let graph = TopologyEngine::generate(&input, self.node_count);
        ctx.graph = Some(graph);

        input
    }
}

pub struct CompoundStage {
    rounds: usize,
}

impl CompoundStage {
    pub fn new(rounds: usize) -> Self {
        Self { rounds }
    }
}

impl EngineStage for CompoundStage {
    fn name(&self) -> &'static str {
        "CompoundStage"
    }

    fn execute(&self, input: Vec<u8>, ctx: &mut PipelineContext) -> Vec<u8> {
        let Some(graph) = ctx.graph.clone() else {
            return input;
        };

        let engine = CompoundEngine::new(graph, self.rounds);
        engine.execute(input, ctx)
    }
}

pub struct RecyclerStage {
    cycles: usize,
}

impl RecyclerStage {
    pub fn new(cycles: usize) -> Self {
        Self { cycles }
    }
}

impl EngineStage for RecyclerStage {
    fn name(&self) -> &'static str {
        "RecyclerStage"
    }

    fn execute(&self, input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        RecyclerEngine::recycle(&input, self.cycles)
    }
}

pub struct MemoryStage {
    memory_mb: usize,
}

impl MemoryStage {
    pub fn new(memory_mb: usize) -> Self {
        Self { memory_mb }
    }
}

impl EngineStage for MemoryStage {
    fn name(&self) -> &'static str {
        "MemoryStage"
    }

    fn execute(&self, input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        MemoryArena::process(&input, self.memory_mb)
    }
}

pub struct FinalizerStage;

impl FinalizerStage {
    pub fn new() -> Self {
        Self
    }
}

impl EngineStage for FinalizerStage {
    fn name(&self) -> &'static str {
        "FinalizerStage"
    }

    fn execute(&self, input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        FinalizerEngine::finalize(&input)
    }
}
