use std::io::{Write, stdout};
use std::time::Duration;

use crate::map::Map;
use crate::coordiante::Coordinate;
use crate::dimensions::Dimensions;
use crate::letter_type::LetterType;
use crate::pixel::Pixel;

use termion::{cursor, screen::AlternateScreen, terminal_size};
pub struct MapManager {
    pub map: Map,
    terminal_dimensions: Dimensions,
    last_written_pos: Vec<Coordinate>,
    border_pos: Vec<Coordinate>

}
impl MapManager {
    pub fn new() -> MapManager {
                //terminal dimension can be obtained through running termion::terminal_size() 
            let terminal_dimensions =  Dimensions {
                    // i dont know why, but w:50, 20, definetly works
                    // other ratios will cause bugs, but i haven't investigated the pattern yet
                    width: 50,
                    height: 20,
                };
        Self {
            last_written_pos: vec![],
            border_pos: vec![],
            terminal_dimensions: terminal_dimensions,
            map: Map::new(terminal_dimensions)
        }
    }
    pub fn init(&mut self){
        self.write_borders();
        self.write_middle_letter('A')
    }
    pub fn draw_map(map: &Map) {

        print!("{}[2J", 27 as char);

        map.vec.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                cursor::Goto(pixel.location.x, pixel.location.y);
                print!("{}", pixel.char);
            });
            println!();
        });
   } 

    pub fn grow(&mut self) {
        if self.last_written_pos.len() > 0 {
            let mut coords_to_check: Vec<Coordinate> = vec![];
            self.last_written_pos.iter().for_each(
                |p| {
                coords_to_check.push(p.clone());
                }
            );
            let mut i = 0;
            while i < self.last_written_pos.len()  {
                self.last_written_pos.remove(i);
                i += 1
            }
            for coord in coords_to_check {
                self.check_surrounding_letters(coord);
            }
        }
    }

    fn write_borders(&mut self) {
        // the border should also be written using the writer
        // but last time i tried doing that, i fucked up everything
        // but it still needs to be done
        // why?
        // so i can write the border points into an array and know where i can't grow any further
        let mut i = 0;

        //top border
        while i <= self.map.vec[0].len() -1 {
            self.writer(Pixel::new(
                Coordinate::new(0 as u16, i as u16),
                't',
                 LetterType::Border
                ));
            i += 1;
        }
 
        i = 0;
        //bottom border
        while i <= self.map.vec[0].len() -1 {
            self.writer(Pixel::new(
                Coordinate::new((self.map.vec.len() -1)  as u16, i as u16),
                'b',
                 LetterType::Border
                ));
            i += 1;
        }
        println!("bottom :3 count :{}", i);

        i = 0;

        println!("left");
        //left border
        while i <= self.map.vec.len() -1 {
            self.writer(Pixel::new(
                Coordinate::new(i  as u16, 0 as u16),
                'l',
                 LetterType::Border
                ));
            i += 1;
        }


        i = 0;
        println!("right");
        //right border 
        while i <= self.map.vec.len() -1 {
            self.writer(Pixel::new(
                Coordinate::new(i  as u16, (self.map.vec[0].len() -1) as u16),
                'r',
                 LetterType::Border
                ));
            i += 1;
        }
    }
    fn write_middle_letter(&mut self, letter: char){
        let x =  self.map.vec.len() /2;
        let y = self.map.vec[x].len() / 2;
        self.writer(Pixel::new(Coordinate::new(x as u16, y as u16),letter, LetterType::Regular));
    }

    fn check_surrounding_letters(&mut self, coords: Coordinate) {

        if self.map.vec[coords.x as usize][coords.y as usize].char == ' ' {
        let mut offset: i32 = -1; 
        let mut i = 0;
        let mut surrounding_letters: Vec<char> = vec![];
        //  if the center letter is already in a corner, i will go out of bounds!!
        /* 
        while i < 4 {
            // horizontal letters                   this prevents going out of bounds
            if i < 2 && (coords.x != 0 || coords.x != (self.map.vec.len() as u16 -1)){
                surrounding_letters.push(
                 self.map.vec[coords.x as usize + offset as usize][coords.y as usize].char
                );

                // vertical  letters
            }else if  {
                surrounding_letters.push(
                 self.map.vec[coords.x as usize][coords.y as usize  + offset as usize].char
                );
            }
            i += 1;
            if offset < 1{
                offset += 2;
            }else{
                offset = 0;
            }
        }
*/
        

        // gather data from sorrounding letters
        // write them to possibility map
        // get own letter
        }
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
    fn writer(&mut self, pixel: Pixel) { 
        self.map.vec[pixel.location.x as usize][pixel.location.y as usize].char = pixel.char;

        match pixel.letter_type {
            LetterType::Border => self.border_pos.push(pixel.location),
            LetterType::Regular => self.last_written_pos.push(pixel.location),
        }
    }
}