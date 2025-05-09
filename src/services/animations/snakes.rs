use crate::{
    map::{map::Map, map_stats::MapStats},
    pixel::{coordiante::Coordinate, letter_type::LetterType, pixel::Pixel},
};

use super::animation_traits::{Grow, Write};

pub struct Snakes {}

impl Write for Snakes {}

impl Grow for Snakes {
    fn grow(&self, map: &mut Map, map_stats: &mut MapStats) {
        if !map_stats.last_written_pos.len() > 0 {
            self.first_step(map, map_stats)
        }
        self.slither(map, map_stats)
    }
}

impl Snakes {
    pub fn new() -> Snakes {
        Snakes {}
    }

    fn slither(&self, map: &mut Map, map_stats: &mut MapStats) {
        let center = Coordinate::new(
            ((map.get_column_len() - 1) / 2) as u32,
            ((map.get_row_len() - 1) / 2) as u32,
        );
        let mut new_pixels: Vec<Pixel> = vec![];

        // check if snake is too close to center
        for pos in &map_stats.last_written_pos {
            if pos.y == center.y - 1 || pos.y == center.y + 1 {
                continue;
            }

            let shift: i32;
            if pos.y > (map.get_row_len() / 2) as u32 {
                shift = -1
            } else {
                shift = 1
            }

            new_pixels.push(Pixel::new(
                Coordinate::new(pos.x, (pos.y as i32 + shift) as u32),
                map.get_pixel(pos).char,
                LetterType::Border,
                map_stats.generation,
            ));
        }

        for pixel in new_pixels {
            self.write(map, Some(&mut map_stats.last_written_pos), pixel);
        }
    }

    fn first_step(&self, map: &mut Map, map_stats: &mut MapStats) {
        let pos_one = Coordinate::new(((map.get_column_len() - 1) / 2) as u32, 0);
        let pos_two = Coordinate::new(
            ((map.get_column_len() - 1) / 2) as u32,
            (map.get_row_len() - 1) as u32,
        );

        let snake_one = Pixel::new(pos_one, 'A', LetterType::Regular, 0);
        let snake_two = Pixel::new(pos_two, 'B', LetterType::Regular, 0);

        self.write(map, Some(&mut map_stats.last_written_pos), snake_one);
        self.write(map, Some(&mut map_stats.last_written_pos), snake_two);
    }
}
