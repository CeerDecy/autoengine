use crate::context::Context;
use crate::types::node::{NodeRunner, NodeRunnerControl, NodeRunnerController, NodeRunnerFactory};
use opencv::core::Mat;
use opencv::imgcodecs;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ImageMatchParams {
    target_score: f32,
    template_image: String,
    source_image: String,
    imread_type: String,
}

pub struct ImageMatchRunner {
    template_image: Option<Mat>,
}

impl ImageMatchRunner {
    pub fn new() -> Self {
        Self {
            template_image: None,
        }
    }
}

impl Default for ImageMatchRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl NodeRunner for ImageMatchRunner {
    type ParamType = ImageMatchParams;

    async fn run(&mut self, ctx: &Context, param: Self::ParamType) -> Result<(), String> {
        let imread_mode = match param.imread_type.to_uppercase().as_str() {
            "GRAYSCALE" => imgcodecs::IMREAD_GRAYSCALE,
            "COLOR" => imgcodecs::IMREAD_COLOR,
            _ => imgcodecs::IMREAD_GRAYSCALE,
        };

        // let template_mat = ctx.load_image_mat(param.template_image.as_str(), imread_mode)?;


        todo!()
    }
}

#[derive(Default)]
pub struct ImageMatchRunnerFactory;

impl ImageMatchRunnerFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodeRunnerFactory for ImageMatchRunnerFactory {
    fn create(&self) -> Box<dyn NodeRunnerControl> {
        Box::new(NodeRunnerController::new(ImageMatchRunner::new()))
    }
}
