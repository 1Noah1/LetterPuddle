mod map_manager;
pub mod coordiante;
pub mod map;
pub mod dimensions;
pub mod pixel;
pub mod letter_type; 
pub mod letter_service; 

use map_manager::MapManager;
use core::time;
use std::{thread::{self}, time::{Instant}};

//use termion::cursor;
// use termion::{input::TermRead, event::Key, raw::IntoRawMode};
// use std::io::{self};

fn main() {
    let mut manager = MapManager::new();
    // render and calculation
    let mut i = 0;

    thread::sleep(time::Duration::from_millis(0));

    MapManager::init(&mut manager);
    loop {
        let start = Instant::now();
        MapManager::draw_map(&mut manager.map);
        MapManager::grow(&mut manager);
        println!("main i: {}", i);
        thread::sleep(time::Duration::from_millis(1000));
        let end = Instant::now();
        println!("time: {:?}", end.duration_since(start));
        if i == 1 {
            break;
        }
        i += 1;
    }
}