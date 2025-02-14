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
    let start = Instant::now();
    loop {
        let start = Instant::now();
        MapManager::draw_map(&mut manager.map);
        MapManager::grow(&mut manager);
        //println!("main i: {}", i);
        //thread::sleep(time::Duration::from_millis(0));
        let end = Instant::now();
        //println!("time: {:?}", end.duration_since(start));
        if i == 10 {
            break;
        }
        i += 1;
    }
    let end = Instant::now();
    print!("total_time: {:?}", end.duration_since(start))
}