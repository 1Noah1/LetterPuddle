use std::collections::HashMap;

use crate::coordiante::Coordinate;
use crate::dimensions::Dimensions;
use crate::letter_service::LetterService;
use crate::letter_type::LetterType;
use crate::map::Map;
use crate::pixel::Pixel;

use colored::{Color, Colorize};
use termion::cursor;
pub struct MapManager {
    pub map: Map,
    // is read by map::new()
    //terminal_dimensions: Dimensions,
    last_written_pos: Vec<Coordinate>,
    // this should save letters, which surrounding letters have already been checked,
    // after they've been assigned a letter
    // this prevents unwanted recursion
    border_pos: HashMap<Coordinate, LetterType>,
    generation: u16,
}

impl MapManager {
    pub fn new() -> MapManager {
        //terminal dimension can be obtained through running termion::terminal_size()
        let terminal_dimensions = Dimensions {
            // i dont know why, but w:50, 20, definetly works
            // other ratios will cause bugs, but i haven't investigated the pattern yet
            width: 90,
            height: 50,
        };
        Self {
            last_written_pos: vec![],
            border_pos: HashMap::new(),
            //terminal_dimensions: terminal_dimensions,
            map: Map::new(terminal_dimensions),
            generation: 0,
        }
    }

    pub fn init(&mut self) {
        self.write_borders();
        self.write_middle_letter('A')
    }

    pub fn draw_map(map: &Map) {
        map.vec.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                cursor::Goto(pixel.location.x, pixel.location.y);

                // no colors
                //print!("{}", pixel.char);

                // print colored letters
                match LetterService::get_colors(pixel.char) {
                    Color::Blue => print!("{}", pixel.char.to_string().blue()),
                    Color::Red => print!("{}", pixel.char.to_string().red()),
                    Color::Magenta => print!("{}", pixel.char.to_string().magenta()),
                    Color::Green => print!("{}", pixel.char.to_string().green()),
                    Color::Cyan => print!("{}", pixel.char.to_string().cyan()),
                    Color::Yellow => print!("{}", pixel.char.to_string().yellow()),
                    _ => print!("{}", pixel.char),
                }

                // one symbol draw
                // let symbol = '#';
                // match LetterService::get_colors(pixel.char) {
                // Color::Blue => print!("{}", symbol.to_string().blue()),
                // Color::Red => print!("{}", symbol.to_string().red()),
                // Color::Magenta => print!("{}", symbol.to_string().magenta()),
                // Color::Green => print!("{}", symbol.to_string().green()),
                // Color::Cyan => print!("{}", symbol.to_string().cyan()),
                // Color::Yellow => print!("{}", symbol.to_string().yellow()),
                // _ => print!("{}", pixel.char)
                // }

                // print color only
                // match LetterService::get_colors(pixel.char) {
                // Color::Blue => print!("{}", " ".to_string().on_blue()),
                // Color::Red => print!("{}", " ".to_string().on_red()),
                // Color::Magenta => print!("{}", " ".to_string().on_magenta()),
                // Color::Green => print!("{}", " ".to_string().on_green()),
                // Color::Cyan => print!("{}", " ".to_string().on_cyan()),
                // Color::Yellow => print!("{}", " ".to_string().on_yellow()),
                // _ => print!("{}", pixel.char)
                // }
            });
            println!();
        });
    }

    fn write_borders(&mut self) {
        // the border should also be written using the writer
        // but last time i tried doing that, i fucked up everything
        // but it still needs to be done
        // why?
        // so i can write the border points into an array and know where i can't grow any further
        let mut i = 0;

        //top border
        while i <= self.map.vec[0].len() - 1 {
            self.writer(Pixel::new(
                Coordinate::new(0 as u16, i as u16),
                '-',
                LetterType::Border,
                0,
            ));
            i += 1;
        }

        i = 0;
        //bottom border
        while i <= self.map.vec[0].len() - 1 {
            self.writer(Pixel::new(
                Coordinate::new((self.map.vec.len() - 1) as u16, i as u16),
                '-',
                LetterType::Border,
                0,
            ));
            i += 1;
        }
        i = 0;

        //left border
        while i <= self.map.vec.len() - 1 {
            self.writer(Pixel::new(
                Coordinate::new(i as u16, 0 as u16),
                '|',
                LetterType::Border,
                0,
            ));
            i += 1;
        }

        i = 0;
        //right border
        while i <= self.map.vec.len() - 1 {
            self.writer(Pixel::new(
                Coordinate::new(i as u16, (self.map.vec[0].len() - 1) as u16),
                '|',
                LetterType::Border,
                0,
            ));
            i += 1;
        }
    }
    fn write_middle_letter(&mut self, letter: char) {
        let x = self.map.vec.len() / 2;
        let y = self.map.vec[x].len() / 2;
        self.writer(Pixel::new(
            Coordinate::new(x as u16, y as u16),
            letter,
            LetterType::Regular,
            0,
        ));
    }

    pub fn grow(&mut self) {
        self.generation += 1;
        if self.last_written_pos.len() > 0 {
            let mut coords_to_check: Vec<Coordinate> = vec![];
            self.last_written_pos.iter().for_each(|p| {
                coords_to_check.push(p.clone());
            });
            let mut i = 0;
            // removing old values
            while i < self.last_written_pos.len() {
                self.last_written_pos.remove(i);
                i += 1
            }
            for coord in coords_to_check {
                self.check_surrounding_letters(coord);
            }
        } else {
            //self.write_middle_letter('A');
        }
    }

    fn check_surrounding_letters(&mut self, coords: Coordinate) {
        if self.map.vec[coords.x as usize][coords.y as usize].char == ' ' {
            let mut surrounding_letters: Vec<char> = vec![];
            if let Some(s) = self.for_each_direction(coords, None) {
                s.iter().for_each(|l| surrounding_letters.push(l.char));
            }
            self.writer(Pixel {
                    location: coords,
                    char: /*LetterService::get_gen_letter(self.generation) */ LetterService::get_letter(&surrounding_letters),
                    letter_type: LetterType::Regular,
                    generation: self.generation
                });

            // write them to possibility map
            // get own letter
        } else {
            self.for_each_direction(coords, Some(&MapManager::check_surrounding_letters));
        }
    }

    fn for_each_direction(
        &mut self,
        coords: Coordinate,
        f: Option<&dyn Fn(&mut MapManager, Coordinate)>,
    ) -> Option<Vec<Pixel>> {
        let mut offset: i32 = -1;
        let mut i = 0;
        let mut values = vec![];

        // TODO: fix: if the center letter is already in a corner, i will go out of bounds!!
        while i < 4 {
            // horizontal letters
            if i < 2 {
                let offset_x = coords.x as i32 + offset;
                match self
                    .border_pos
                    .get(&Coordinate::new(offset_x as u16, coords.y))
                {
                    None => match f {
                        Some(f) => {
                            if self.map.vec[offset_x as usize][coords.y as usize].generation
                                > self.generation
                            {
                                f(
                                    self,
                                    Coordinate {
                                        x: (coords.x as i32 + offset) as u16,
                                        y: coords.y,
                                    },
                                )
                            }
                        }
                        None => values.push(
                            self.map.vec[(coords.x as i32 + offset) as usize][coords.y as usize],
                        ),
                    },
                    _ => { /*border hit*/ }
                }
            // vertical  letters
            } else {
                let offset_y = coords.y as i32 + offset;
                match self.border_pos.get(&Coordinate {
                    x: coords.x,
                    y: (offset_y) as u16,
                }) {
                    None => match f {
                        Some(f) => {
                            if self.map.vec[coords.x as usize][offset_y as usize].generation
                                > self.generation
                            {
                                f(
                                    self,
                                    Coordinate {
                                        x: coords.x,
                                        y: offset_y as u16,
                                    },
                                );
                            }
                        }
                        None => values.push(self.map.vec[coords.x as usize][offset_y as usize]),
                    },
                    _ => { /*border hit*/ }
                }
            }
            i += 1;
            if offset < 1 {
                offset += 2;
            } else {
                offset = -1;
            }
        }
        match f {
            Some(_) => return None,
            None => return Some(values),
        }
    }

    fn writer(&mut self, pixel: Pixel) {
        self.map.vec[pixel.location.x as usize][pixel.location.y as usize] = pixel;
        //thread::sleep(time::Duration::from_millis(0));
        match pixel.letter_type {
            LetterType::Border => {
                self.border_pos.insert(pixel.location, pixel.letter_type);
            }
            LetterType::Regular => self.last_written_pos.push(pixel.location),
        }
    }
}
