use crate::{
    config::{animation::Animation, general_config::GeneralConfig, platform::Platform},
    map::{map::Map, map_manager::MapManager},
    render::{render_engine::RenderEngine, terminal_render_engine::TerminalRenderEngine},
    services::{
        animations::{alphabet::Alphabet, islands::Island, snakes::Snakes},
        dimensions_service::DimensionsService,
    },
};

pub struct AppManager {}

impl AppManager {
    pub fn new() -> AppManager {
        AppManager {}
    }
    pub fn run(self) {
        let config = GeneralConfig::config_from_user_preference();
        let map = Map::new(DimensionsService::get_dimensions());

        let mut manager: MapManager = match config.animation {
            Animation::Alphabet => MapManager::new(map, Box::new(Alphabet::new())),
            Animation::Islands => MapManager::new(map, Box::new(Island::new())),
            Animation::Snakes => MapManager::new(map, Box::new(Snakes::new())),
        };

        let mut engine = match config.render_config.platform {
            Platform::Terminal => TerminalRenderEngine::new(config.render_config),
        };
        manager.run();

        // calculation thread
        let mut i = 0;
        loop {
            manager.run();
            engine.add_frame(manager.map.clone());
            if i == 68 {
                break;
            }
            i += 1;
        }

        // render thread
        while engine.frames_left() {
            engine.render();
        }
    }
}
