use std::collections::HashMap;

use crate::map::Map;
use crate::coordiante::Coordinate;
use crate::dimensions::Dimensions;
use crate::letter_type::LetterType;
use crate::pixel::Pixel;

use termion::cursor;
pub struct MapManager {
    pub map: Map,
    terminal_dimensions: Dimensions,
    last_written_pos: Vec<Coordinate>,
    border_pos: HashMap<Coordinate, LetterType>

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
            border_pos: HashMap::new(),
            terminal_dimensions: terminal_dimensions,
            map: Map::new(terminal_dimensions)
        }
    }

    pub fn init(&mut self){
        self.write_borders();
        self.write_middle_letter('A')
    }

    pub fn draw_map(map: &Map) {

        // what is this??
        //print!("{}[2J", 27 as char);

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
            // removing values old values
            while i < self.last_written_pos.len()  {
                self.last_written_pos.remove(i);
                i += 1
            }
            for coord in coords_to_check {
                self.check_surrounding_letters(coord);
            }
        }else{
            self.write_middle_letter('A');
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
        i = 0;

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
        // this function would cause endless recursion till the whole map is filled out and even beyond that
        // why??
        // when a value lets say we have the middle letter set
        // surrounding letters are checked and filled out
        // works fine
        // Now we have a situation where a surrounding letter is not empty 
        //
        //
        //      B
        //     BAB
        //      B
        // in this case all B's will go to A and check it surrounding letters again.
        // Two possible solutions for this issue
        // First:
        //  hold a collection of every written value ever, seperate from last_written_pos
        //  check if the not empty value is in this collection
        // Second: 
        //  parrallel rendering 
        //  rendering the map parallel to it being proccessed 
        //  advantage:
        //      it would visually more dynamic (every step wouldn't completly lookl the same)
        //      i would learn more about threading
        //  disadvantage:
        //      i would have to learn about threading and reading data while it is being modified
        //          i could probably just make a copy, render off of that and then throw it away



        if self.map.vec[coords.x as usize][coords.y as usize].char == ' ' {
            //let surrounding_letter = self.for_each_direction(coords, None);

            // write them to possibility map
            // get own letter
        }else {
            self.for_each_direction(
                coords,
                 Some(&MapManager::check_surrounding_letters)
                );
        }
    }
    
    fn for_each_direction(
        &mut self,coords: Coordinate,
        f: Option<&dyn Fn(&mut MapManager, Coordinate)>
    ) -> Option<Vec<Pixel>> {
        let mut offset: i32 = -1; 
        let mut i = 0;
        let mut values = vec![];

        //  if the center letter is already in a corner, i will go out of bounds!!
        while i < 4 {
            // horizontal letters
            if i < 2 {
                match self.border_pos.get(&Coordinate::new(coords.x +  offset as u16, coords.y)) {
                    None =>  {
                        match f {
                            Some(f) => {
                                f(self, Coordinate{x: coords.x + offset as u16, y: coords.y})
                            },
                            None => values.push(self.map.vec[(coords.x + offset as u16)as usize][coords.y as usize])
                        }
                    }
                    _ => {/*border hit*/}
                }
            // vertical  letters
            }else {
                match self.border_pos.get(&Coordinate{x: coords.x, y: coords.y +  offset as u16}) {
                    None =>  {
                        match f {
                            Some(f) => {
                                f(self, Coordinate{x: coords.x, y: coords.y  + offset as u16});
                            },
                            None => values.push(self.map.vec[coords.x as usize][(coords.y + offset as u16)as usize])
                        }
                    }
                    _ => {/*border hit*/}
                }
            }
            i += 1;
            if offset < 1{
                offset += 2;
            }else{
                offset = 0;
            }
            
        }
        match f {
                Some(_) => return None,
                None => return Some(values)
        }
    }


    fn writer(&mut self, pixel: Pixel) { 
        self.map.vec[pixel.location.x as usize][pixel.location.y as usize].char = pixel.char;

        match pixel.letter_type {
            LetterType::Border => {self.border_pos.insert(pixel.location, pixel.letter_type);}
            LetterType::Regular => self.last_written_pos.push(pixel.location)
        }
    }
}