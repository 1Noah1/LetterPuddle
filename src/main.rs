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
  //  let handle = thread::spawn(|| {
  //      // Enable raw mode so that key events can be captured without pressing enter
  //      let _stdout = io::stdout().into_raw_mode().unwrap();
  //      // Create a handle for standard input (stdin)
  //      let mut  stdin = io::stdin().lock().keys();
  //
  //      // this causes shift in letters
  //      // loop {
  //          // if let Some(Ok(Key::Esc)) = stdin.next() {
  //              // panic!("Esc key pressed. Exiting loop.");
  //          // }
  //          // thread::sleep(time::Duration::from_millis(10000));
  //      // }
  //  });

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
        


    //MapManager::write_borders(&mut manager.map);

    // Wait for the thread to finish
   // handle.join().unwrap();
}