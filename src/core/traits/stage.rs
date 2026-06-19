use crate::pipeline::context::PipelineContext;

pub trait EngineStage {
    fn name(&self) -> &'static str;

    fn execute(
        &self,
        input: Vec<u8>,
        ctx: &mut PipelineContext,
    ) -> Vec<u8>;
}
