use crate::{config::render_config::RenderConfig, map::map::Map};

pub trait RenderEngine{
    fn new(config: RenderConfig) -> impl RenderEngine;
    fn render(&mut self);
    fn add_frame(&mut self, map: Map);
    fn frames_left(&self) -> bool;
}
