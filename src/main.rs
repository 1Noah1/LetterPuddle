pub mod coordiante;
pub mod dimensions;
pub mod letter_service;
pub mod letter_type;
pub mod map;
mod map_manager;
pub mod pixel;

use core::time;
use map_manager::MapManager;
use std::{
    thread::{self},
    time::Instant,
};

fn main() {
    let mut manager = MapManager::new();

    let mut i = 0;
    MapManager::init(&mut manager);
    let start = Instant::now();
    loop {
        let start = Instant::now();
        MapManager::draw_map(&mut manager.map);
        MapManager::grow(&mut manager);
        thread::sleep(time::Duration::from_millis(100));
        let end = Instant::now();
        println!("time: {:?}", end.duration_since(start));
        if i == 80 {
            break;
        }
        i += 1;
    }
    let end = Instant::now();
    print!("total_time: {:?}", end.duration_since(start))
}
