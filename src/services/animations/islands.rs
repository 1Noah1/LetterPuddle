use crate::{
    map::{map::Map, map_stats::MapStats},
    pixel::{coordiante::Coordinate, letter_type::LetterType, pixel::Pixel},
    services::letter_service::LetterService,
};

use super::animation_traits::{Grow, Write};

pub struct Island {}

impl Write for Island {}

impl Grow for Island {
    fn grow(&self, map: &mut Map, map_stats: &mut MapStats) {
        if map_stats.last_written_pos.len() > 0 {
            let mut coords_to_check: Vec<Coordinate> = vec![];
            map_stats.last_written_pos.iter().for_each(|p| {
                coords_to_check.push(p.clone());
            });
            let mut i = 0;
            // removing old values
            while i < map_stats.last_written_pos.len() {
                map_stats.last_written_pos.remove(i);
                i += 1
            }
            for coord in coords_to_check {
                self.check_surrounding_letters(map, map_stats, &coord);
            }
        } else {
            self.write_borders(map);
            self.write(
                map,
                Some(&mut map_stats.last_written_pos),
                self.get_middle_letter(map, 'A'),
            );
        }
    }
}
impl Island {
    pub fn new() -> Island {
        Island {}
    }
    fn get_middle_letter(&self, map: &Map, letter: char) -> Pixel {
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

    /*
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
    */

    fn check_surrounding_letters(
        &self,
        map: &mut Map,
        map_stats: &mut MapStats,
        coords: &Coordinate,
    ) {
        if map.get_ref_pixel(coords).char == ' ' {
            let mut surrounding_letters: Vec<char> = vec![];
            if let Some(s) = self.for_each_direction(map, map_stats, coords, None) {
                s.iter().for_each(|l| surrounding_letters.push(l.char));
            }
            let letter = LetterService::get_letter(&surrounding_letters);
            Island::write(
                &self,
                map,
                Some(&mut map_stats.last_written_pos),
                Pixel::new(*coords, letter, LetterType::Regular, map_stats.generation),
            );
        } else {
            self.for_each_direction(
                map,
                map_stats,
                &coords,
                Some(&Island::check_surrounding_letters),
            );
        }
    }

    fn for_each_direction(
        &self,
        map: &mut Map,
        map_stats: &mut MapStats,
        coords: &Coordinate,
        f: Option<&dyn Fn(&Island, &mut Map, &mut MapStats, &Coordinate)>,
    ) -> Option<Vec<Pixel>> {
        let mut offset: i32 = -1;
        let mut i = 0;
        let mut values = vec![];

        // TODO: fix: if the center letter is already in a corner, i will go out of bounds!!
        while i < 4 {
            // horizontal letters
            if i < 2 {
                let offset_x = coords.x as i32 + offset;
                if !map.is_border_pos(Coordinate::new(offset_x as u32, coords.y)) {
                    match f {
                        Some(f) => {
                            if map
                                .get_pixel(&Coordinate {
                                    x: offset_x as u32,
                                    y: coords.y,
                                })
                                .generation
                                > map_stats.generation
                            {
                                f(
                                    self,
                                    map,
                                    map_stats,
                                    &Coordinate {
                                        x: (coords.x as i32 + offset) as u32,
                                        y: coords.y,
                                    },
                                )
                            }
                        }
                        None => values.push(
                            // i think i should use references instead
                            // but that causes a weird error i dont understand yet
                            map.get_pixel(&Coordinate::new(
                                (coords.x as i32 + offset) as u32,
                                coords.y,
                            )),
                        ),
                    }
                }

            // vertical  letters
            } else {
                let offset_y = coords.y as i32 + offset;
                if map.is_border_pos(Coordinate::new(coords.x, (offset_y) as u32)) {
                    match f {
                        Some(f) => {
                            if map
                                .get_pixel(&Coordinate::new(coords.x, offset_y as u32))
                                .generation
                                > map_stats.generation
                            {
                                f(
                                    self,
                                    map,
                                    map_stats,
                                    &Coordinate {
                                        x: coords.x,
                                        y: offset_y as u32,
                                    },
                                );
                            }
                        }
                        None => values.push(
                            // i think i should use references instead
                            // but that causes a weird error i dont understand yet
                            map.get_pixel(&Coordinate::new(coords.x, offset_y as u32)),
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

    // use the write trait instead
    /*
        fn writer(&mut self, pixel: Pixel) {
            self.map.set_pixel(pixel);
            match pixel.letter_type {
                LetterType::Border => {
                    self.map.add_to_border(pixel.location);
                }
                LetterType::Regular => self.last_written_pos.push(pixel.location),
            }
        }
    */
}
