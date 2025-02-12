mod map_manager;
pub mod coordiante;
pub mod map;
pub mod dimensions;
pub mod pixel;
pub mod letter_type; 


use map_manager::MapManager;
use core::time;
use std::thread::{self};

//use termion::cursor;
// use termion::{input::TermRead, event::Key, raw::IntoRawMode};
// use std::io::{self};

fn main() {
    let mut manager = MapManager::new();
    // render and calculation
    let mut i = 0;

    MapManager::init(&mut manager);
    loop {
        MapManager::draw_map(&mut manager.map);
        // MapManager::grow(&mut manager);
        thread::sleep(time::Duration::from_millis(1000));
        if i <= 10 {
            break;
        }
        i += 1;
    }
}