use crate::render_config::RenderConfig;
use crate::map::Map;

pub trait Render {
    fn render(config: &RenderConfig, map: &Map);
}