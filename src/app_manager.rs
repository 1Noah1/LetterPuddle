use std::{thread, time::{self, Instant}};

use crate::{config::{animation::{self, Animation}, general_config::GeneralConfig}, map::map_manager::MapManager, render::{render_engine::RenderEngine, terminal_render_engine::TerminalRenderEngine}, services::animations::{alphabet::Alphabet, grow::Grow, islands}};

pub struct AppManager {

}

impl AppManager {
    pub fn new() -> AppManager{
        AppManager {}
    }
    pub fn run(self) {
    let mut i = 0;
    let config = GeneralConfig::config_from_user_preference();
    match config.animation {
        Animation::Alphabet => MapManager::new(config, Alphabet::new())
        Animation::Islands => MapManager::new(config, Islands::new())
        Animation::Snakes => MapManager::new(config, Snakes::new())

    }
    let mut manager = MapManager::new(&config, );
    let mut engine = TerminalRenderEngine::new(config.render_config,);

    MapManager::init(&mut manager);
    let start = Instant::now();

    loop {
        let start = Instant::now();
        engine.render();
        // TerminalRenderEngine::render(&config.render_config, &mut manager.map);

        manager.grow();
        thread::sleep(time::Duration::from_millis(50));
        engine.add_frame(manager.map.clone());

        let end = Instant::now();
        println!("time: {:?}", end.duration_since(start));
        if i == 68 {
            break;
        }
        i += 1;
    }

    let end = Instant::now();
    print!("total_time: {:?}", end.duration_since(start));
    }
}