use crate::{context::Context, types::node::NodeRunner};

pub struct StartRunner;

pub struct Params;

impl NodeRunner<Params> for StartRunner {
    fn run(&self, _ctx: Context, _param: Params) -> Result<(), String> {
        // nothing need to do
        Ok(())
    }
}
