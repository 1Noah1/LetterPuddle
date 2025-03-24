use crate::{
    map::{map::Map, map_stats::MapStats},
    pixel::{coordiante::Coordinate, letter_type::LetterType, pixel::Pixel},
};

pub trait Write {
    fn write(&self, map: &mut Map, last_written_pos: &mut Vec<Coordinate>, pixel: Pixel) {
        map.set_pixel(pixel);
        match pixel.letter_type {
            LetterType::Border => {
                map.add_to_border(pixel.location);
            }
            LetterType::Regular => last_written_pos.push(pixel.location),
        }
    }
}
pub trait Grow {
    // return true if frame has been calculated
    fn grow(&self, map: &mut Map, map_stats: &mut MapStats);
}
