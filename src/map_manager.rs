use crate::dimensions::Dimensions;
use crate::letter_service::LetterService;
use crate::letter_type::LetterType;
use crate::map::Map;
use crate::pixel::Pixel;
use crate::{config::Config, coordiante::Coordinate};

use colored::{Color, Colorize};
use termion::cursor;
pub struct MapManager {
    pub map: Map,
    // is read by map::new()
    //terminal_dimensions: Dimensions,
    last_written_pos: Vec<Coordinate>,
    generation: u32,
    config: Config,
}

impl MapManager {
    pub fn new(config: &Config) -> MapManager {
        //terminal dimension can be obtained through running termion::terminal_size()
        let terminal_dimensions = Dimensions {
            // i dont know why, but w:50, 20, definetly works
            // other ratios will cause bugs, but i haven't investigated the pattern yet
            width: 90,
            height: 50,
        };
        Self {
            last_written_pos: vec![],
            //terminal_dimensions: terminal_dimensions,
            map: Map::new(terminal_dimensions),
            generation: 0,
            config: config.clone(),
        }
    }

    pub fn init(&mut self) {
        self.write_borders();
        self.write_middle_letter('A')
    }

    pub fn draw_map(config: &Config, map: &Map) {
        map.vec.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                cursor::Goto(pixel.location.x as u16, pixel.location.y as u16);


                if config.render_letters {
                    // print  letters
                    match pixel.color {
                        Color::Blue => print!("{}", pixel.char.to_string().blue()),
                        Color::Red => print!("{}", pixel.char.to_string().red()),
                        Color::Magenta => print!("{}", pixel.char.to_string().magenta()),
                        Color::Green => print!("{}", pixel.char.to_string().green()),
                        Color::Cyan => print!("{}", pixel.char.to_string().cyan()),
                        Color::Yellow => print!("{}", pixel.char.to_string().yellow()),
                        _ => print!("{}", pixel.char.to_string().white()),
                    }
                } else {
                    // print color only

                    match pixel.color {
                        Color::Blue => print!("{}", " ".to_string().on_blue()),
                        Color::Red => print!("{}", " ".to_string().on_red()),
                        Color::Magenta => print!("{}", " ".to_string().on_magenta()),
                        Color::Green => print!("{}", " ".to_string().on_green()),
                        Color::Cyan => print!("{}", " ".to_string().on_cyan()),
                        Color::Yellow => print!("{}", " ".to_string().on_yellow()),
                        _ => print!("{}", pixel.char),
                    }
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
            });
            println!();
        });
    }

    fn write_borders(&mut self) {
        //tried reducing lines by looping more
        // i think i barely improved it lol

        let right_left = '|';
        let top_bottom = '-';

        //top and bottom border
        let mut j = 0;
        while j < 2 {
            let mut i = 0;
            while i <= self.map.get_row_len() - 1 {
                let x;
                let y;
                if j < 1 {
                    x = 0;
                    y = i;
                } else {
                    x = self.map.get_column_len() - 1;
                    y = i;
                }
                self.writer(Pixel::new(
                    Coordinate::new(x as u32, y as u32),
                    top_bottom,
                    LetterType::Border,
                    0,
                    true,
                    Color::White,
                ));
                i += 1;
            }
            j += 1;
        }

        //left and right border
        let mut j = 0;
        while j < 2 {
            let mut i = 0;
            while i <= self.map.get_column_len() - 1 {
                let x;
                let y;
                if j < 1 {
                    x = i;
                    y = 0;
                } else {
                    x = i;
                    y = self.map.get_row_len() - 1;
                }
                self.writer(Pixel::new(
                    Coordinate::new(x as u32, y as u32),
                    right_left,
                    LetterType::Border,
                    0,
                    true,
                    Color::White,
                ));
                i += 1;
            }
            j += 1;
        }
    }
    fn write_middle_letter(&mut self, letter: char) {
        self.writer(Pixel::new(
            Coordinate::new(
                (self.map.get_column_len() / 2) as u32,
                (self.map.get_row_len() / 2) as u32,
            ),
            letter,
            LetterType::Regular,
            0,
            self.config.render_letters,
            if self.config.colored {
                LetterService::get_color(letter)
            } else {
                Color::White
            },
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
            // case should not be hit if innit was performed
            //self.write_middle_letter('A');
        }
    }

    fn check_surrounding_letters(&mut self, coords: Coordinate) {
        if self.map.get_pixel(coords).char == ' ' {
            let mut surrounding_letters: Vec<char> = vec![];
            if let Some(s) = self.for_each_direction(coords, None) {
                s.iter().for_each(|l| surrounding_letters.push(l.char));
            }
            let letter = if self.config.iterative_letters {
                LetterService::get_gen_letter(self.generation)
            } else {
                LetterService::get_letter(&surrounding_letters)
            };
            self.writer(Pixel::new(
                coords,
                letter,
                LetterType::Regular,
                self.generation,
                self.config.render_letters,
                if self.config.colored {
                    LetterService::get_color(letter)
                } else {
                    Color::White
                },
            ));
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
                if !self
                    .map
                    .is_border_pos(Coordinate::new(offset_x as u32, coords.y))
                {
                    match f {
                        Some(f) => {
                            if self
                                .map
                                .get_pixel(Coordinate::new(offset_x as u32, coords.y))
                                .generation
                                > self.generation
                            {
                                f(
                                    self,
                                    Coordinate {
                                        x: (coords.x as i32 + offset) as u32,
                                        y: coords.y,
                                    },
                                )
                            }
                        }
                        None => values.push(self.map.get_pixel(Coordinate::new(
                            (coords.x as i32 + offset) as u32,
                            coords.y,
                        ))),
                    }
                }

            // vertical  letters
            } else {
                let offset_y = coords.y as i32 + offset;
                if !self
                    .map
                    .is_border_pos(Coordinate::new(coords.x, (offset_y) as u32))
                {
                    match f {
                        Some(f) => {
                            if self
                                .map
                                .get_pixel(Coordinate::new(coords.x, offset_y as u32))
                                .generation
                                > self.generation
                            {
                                f(
                                    self,
                                    Coordinate {
                                        x: coords.x,
                                        y: offset_y as u32,
                                    },
                                );
                            }
                        }
                        None => values.push(
                            self.map
                                .get_pixel(Coordinate::new(coords.x, offset_y as u32)),
                        ),
                    }
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
        self.map.set_pixel(pixel);
        match pixel.letter_type {
            LetterType::Border => {
                self.map.add_to_border(pixel.location);
            }
            LetterType::Regular => self.last_written_pos.push(pixel.location),
        }
    }
}
