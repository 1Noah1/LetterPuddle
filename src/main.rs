pub mod config;
pub mod map;
pub mod pixel;

use core::time;
use std::{
    thread::{self},
    time::Instant,
};

fn main() {
    let mut i = 0;
    let config = Config::config_from_user_preference();
    let mut manager = MapManager::new(&config);
    let engine = TerminalRenderEngine::new(config.render_config,);

    MapManager::init(&mut manager);
    let start = Instant::now();

    loop {
        let start = Instant::now();


        // TerminalRenderEngine::render(&config.render_config, &mut manager.map);

        manager.grow();
        thread::sleep(time::Duration::from_millis(50));
        engine.add_frame(manager.map);
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
