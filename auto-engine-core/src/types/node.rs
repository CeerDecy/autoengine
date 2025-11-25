use schemars::Schema;

use crate::context::Context;

pub struct NodeName {
    pub zh: String,
    pub en: String,
}

pub trait NodeDefine<T, R>
where
    R: NodeRunner<T>,
{
    fn action_type(&self) -> String;

    fn name(&self) -> NodeName;

    fn icon(&self) -> String;

    fn output_schema(&self) -> Schema;

    fn input_schema(&self) -> Schema;

    fn node_runner(&self) -> R;
}

pub trait NodeRunner<T> {
    fn run(&self, ctx: Context, param: T) -> Result<(), String>;
}
