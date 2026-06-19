use crate::topology::Graph;

#[derive(Default)]
pub struct PipelineContext {
    pub graph: Option<Graph>,
}
