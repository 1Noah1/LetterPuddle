pub mod config;
pub mod coordiante;
pub mod dimensions;
pub mod letter_service;
pub mod letter_type;
pub mod map;
mod map_manager;
pub mod pixel;
pub mod render;

use crate::config::Config;
use core::time;
use map_manager::MapManager;
use render::Render;
use std::{
    thread::{self},
    time::Instant,
};

fn main() {
    let mut i = 0;
    let config = Config::user_preference();
    let mut manager = MapManager::new(&config);

    MapManager::init(&mut manager);
    let start = Instant::now();
    loop {
        let start = Instant::now();
        Render::draw_map(&config, &mut manager.map);
        MapManager::grow(&mut manager);
        thread::sleep(time::Duration::from_millis(50));
        let end = Instant::now();
        println!("time: {:?}", end.duration_since(start));
        if i == 68 {
            break;
        }
        i += 1;
    }
    let end = Instant::now();
    print!("total_time: {:?}", end.duration_since(start))
}
