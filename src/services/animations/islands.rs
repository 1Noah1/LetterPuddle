use crate::{map::map::Map, pixel::{coordiante::Coordinate, letter_type::LetterType, pixel::Pixel}};

use super::grow::Grow;

struct Islands {}

impl Grow for Islands {
    fn grow(map: &mut Map){

    }
}
impl Islands {
    fn new() -> Islands{
        Islands {}
    }


    fn write_middle_letter(map: Map, letter: char) -> Pixel{
        Pixel::new(
            Coordinate::new(
                (map.get_column_len() / 2) as u32,
                (map.get_row_len() / 2) as u32,
            ),
            letter,
            LetterType::Regular,
            0,
        )
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
                        None => values.push(
                            // i think i should use references instead
                            // but that causes a weird error i dont understand yet
                            *self.map.get_pixel(Coordinate::new(
                                (coords.x as i32 + offset) as u32,
                                coords.y,
                            )),
                        ),
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
                            // i think i should use references instead
                            // but that causes a weird error i dont understand yet
                            *self
                                .map
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
}