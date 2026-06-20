use crate::compound::CompoundEngine;
use crate::chaos::ChaosEngine;
use crate::entropy::EntropyVector;
use crate::finalizer::FinalizerEngine;
use crate::memory::MemoryArena;
use crate::recycler::RecyclerEngine;
use crate::seed::SeedEngine;
use crate::topology::TopologyEngine;
use crate::traits::stage::EngineStage;

use super::context::PipelineContext;

// ─────────────────────────────────────────────────────────────────────────────
// SeedStage
// ─────────────────────────────────────────────────────────────────────────────

pub struct SeedStage {
    entropy: EntropyVector,
    /// Optional server-side pepper. If Some, it is mixed into the seed hash
    /// after the salt with a domain separator. Not stored in the hash string.
    pepper: Option<Vec<u8>>,
}

impl SeedStage {
    pub fn new(entropy: EntropyVector) -> Self {
        Self { entropy, pepper: None }
    }

    pub fn with_pepper(entropy: EntropyVector, pepper: Vec<u8>) -> Self {
        Self { entropy, pepper: Some(pepper) }
    }
}

impl EngineStage for SeedStage {
    fn name(&self) -> &'static str {
        "SeedStage"
    }

    fn execute(&self, input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        let seed_engine = SeedEngine::new();
        seed_engine
            .generate(&input, &self.entropy, self.pepper.as_deref())
            .expect("seed generation is infallible")
            .root
            .to_vec()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TopologyStage
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
// CompoundStage
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
// ChaosStage
// ─────────────────────────────────────────────────────────────────────────────

pub struct ChaosStage {
    engine: ChaosEngine,
}

impl ChaosStage {
    pub fn new() -> Self {
        Self {
            engine: ChaosEngine::new(),
        }
    }
}

impl EngineStage for ChaosStage {
    fn name(&self) -> &'static str {
        "ChaosStage"
    }

    fn execute(&self, input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        self.engine.transform(&input)
    }
}

impl Default for ChaosStage {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RecyclerStage
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
// MemoryStage
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
// FinalizerStage
// ─────────────────────────────────────────────────────────────────────────────

pub struct FinalizerStage {
    /// Output byte length — 64 for Standard, 128 for Quantum.
    output_size: usize,
}

impl FinalizerStage {
    pub fn new(output_size: usize) -> Self {
        Self { output_size }
    }
}

impl EngineStage for FinalizerStage {
    fn name(&self) -> &'static str {
        "FinalizerStage"
    }

    fn execute(&self, input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        FinalizerEngine::finalize(&input, self.output_size)
    }
}
