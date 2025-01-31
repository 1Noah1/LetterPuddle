use core::time;
use std::thread::{self, Thread};

use termion::cursor;
use termion::{input::TermRead, event::Key, raw::IntoRawMode};
use std::io::{self};

fn main() {


    let handle = thread::spawn(|| {
        // Enable raw mode so that key events can be captured without pressing enter
        let stdout = io::stdout().into_raw_mode().unwrap();
        // Create a handle for standard input (stdin)
        let mut  stdin = io::stdin().lock().keys();

        loop {
            if let Some(Ok(Key::Esc)) = stdin.next() {
                panic!("Esc key pressed. Exiting loop.");
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    let mut manager = MapManager::new();
    // render and calculation
    let mut i = 0;

    MapManager::init(&mut manager);
    loop {
        MapManager::draw_map(&mut manager.map);
        MapManager::grow(&mut manager);
        thread::sleep(time::Duration::from_millis(1000));
        if i <= 10 {
            break;
        }
        i += 1;
    }
        


    //MapManager::write_borders(&mut manager.map);

    // Wait for the thread to finish
    handle.join().unwrap();
}

struct MapManager {
    map: Map,
    terminal_dimensions: Dimensions,
    last_written_pos: Vec<Coordinate>
}
impl MapManager {
    fn new() -> MapManager {
                //terminal dimension can be obtained through running termion::terminal_size() 
            let terminal_dimensions =  Dimensions {
                    // i dont know why, but w:50, 20, definetly works
                    // other ratios will cause bugs, but i haven't investigated the pattern yet
                    width: 50,
                    height: 20,
                };
        Self {
            last_written_pos: vec![],
            terminal_dimensions: terminal_dimensions,
            map: Map::new(terminal_dimensions)
        }
    }
    fn init(&mut self){
        self.write_borders();
        self.write_middle_letter('A')

    }
    fn draw_map(map: &Map) {
        map.vec.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                cursor::Goto(pixel.location.x, pixel.location.y);
                print!("{}", pixel.char);
            });
            println!();
        });
   } 

    fn grow(&self) {
        if self.last_written_pos.len() > 0 {
        let mut coords_to_check: Vec<Coordinate> = vec![];
        for mut pos in self.last_written_pos {
            // max value counts as deleted, so i know which ones to ignore
            coords_to_check.push(pos.clone());
            pos.x = u16::max_value();
            pos.y = u16::max_value();
        }
        for coord in coords_to_check {
            self.check_surrounding_letters(coord.clone());
        }
        }
    }

    fn write_borders(&mut self) {
        //top border
        self.map.vec.first_mut().unwrap().iter_mut().for_each(
            |unit|{
                unit.char = 't';
            }
        );
        //bottom border
        self.map.vec.last_mut().unwrap().iter_mut().for_each(
            |unit|{
                unit.char = 'b';
            }
        );
        //left border
        self.map.vec.iter_mut().for_each(
            |row|{
                row.first_mut().unwrap().char = 'l'
            }
        );
        //right border 
        self.map.vec.iter_mut().for_each(
            |row|{
                row.last_mut().unwrap().char = 'r'
            }
        );
    }
    fn write_middle_letter(mut self, letter: char){
        let x =  self.map.vec.len() /2;
        let y = self.map.vec[x].len() / 2;
        self.writer(Coordinate::new(x as u16, y as u16),letter);
    }

    fn check_surrounding_letters(&mut self, coords: Coordinate) {
        let column_middle =  self.map.vec.len() /2;
        let row_middle = self.map.vec[column_middle].len() / 2;
        println!("Coords: {:?}", coords);
        println!("indexes:  column: {:?} , row: {:?}", column_middle, row_middle);
        // if (self is empty) {
        // gather data from sorrounding letters
        // write them to possibility map
        // get own letter
        //}
        //else {
        // move imaginary cursor left from origin
        //check_sorrounding_letters();
        // move imaginary cursor right from origin
        //check_sorrounding_letters();
        // move imaginary cursor up from origin
        //check_sorrounding_letters();
        // move imaginary cursor down from origin
        //check_sorrounding_letters();
        //}
    }

    // the pos that is written to, ink the pix that is written
    fn writer(&mut self,  pos: Coordinate, letter: char) {
        self.map.vec[pos.x as usize][pos.y as usize].char = letter;

        self.last_written_pos.push(pos);
    }
}

struct Map{
    vec: Vec<Vec<Pixel>>,
}
impl Map {
    fn new(dimensions: Dimensions, ) -> Map {
        println!("{:?}", dimensions);
        let mut vec: Vec<Vec<Pixel>> = vec![
            vec![Pixel::new(Coordinate::new(0, 0), ' '); dimensions.width as usize];
            dimensions.height as usize
        ];

        //<---- assign coordinates ---->
        let mut i = 0;
        for row in vec.iter_mut() {
            let mut j = 0;
            for pixel in row.iter_mut() {
                pixel.location.x = i as u16;
                pixel.location.y = j as u16;
                 j += 1;
            }
            i += 1;
        }
        //<---- assign coordinates ---->

        Map {
            vec: vec,
        }
    }
}

#[derive(Debug,Clone, Copy)]
struct Pixel {
    location: Coordinate,
    char: char,
}

impl Pixel {
    fn new(location: Coordinate, char: char) -> Pixel {
        Pixel {
            location: location,
            char: char,
        }
    }
}

#[derive(Debug,Clone,Copy)]
struct Dimensions {
    width: u16,
    height: u16,
}

#[derive(Debug,Clone,Copy)]
struct Coordinate {
    x: u16,
    y: u16,
}
impl Coordinate {
    fn new(x: u16, y: u16) -> Coordinate {
        Coordinate {
            x: x,
            y: y,
        }
    }
}
