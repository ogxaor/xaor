use xcrypt::pipeline::runner::PipelineRunner;
use xcrypt::pipeline::context::PipelineContext;
use xcrypt::traits::stage::EngineStage;

struct DummyStage;

impl EngineStage for DummyStage {
    fn name(&self) -> &'static str {
        "DummyStage"
    }

    fn execute(&self, mut input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        input.push(99);
        input
    }
}

fn main() {
    let mut pipeline = PipelineRunner::new();

    pipeline.add_stage(Box::new(DummyStage));

    let input = vec![1, 2, 3];
    let output = pipeline.run(input);

    println!("{:?}", output);
}
