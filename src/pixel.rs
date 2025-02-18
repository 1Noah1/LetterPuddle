use crate::{coordiante::Coordinate, letter_type::LetterType};

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub location: Coordinate,
    pub char: char,
    pub letter_type: LetterType,
    pub generation: u16,
}

impl Pixel {
    pub fn new(
        location: Coordinate,
        char: char,
        letter_type: LetterType,
        generation: u16,
    ) -> Pixel {
        Pixel {
            location: location,
            char: char,
            letter_type: letter_type,
            generation: generation,
        }
    }
}
