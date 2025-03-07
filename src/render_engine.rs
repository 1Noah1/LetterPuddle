use crate::render_config::RenderConfig;
use crate::map::{self, Map};

pub trait RenderEngine {
    fn new (config: &RenderConfig);
    fn render(&mut self);
    fn add_frame(&mut self, map: &Map);
}