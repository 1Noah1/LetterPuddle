use colored::Color;

use crate::{coordiante::Coordinate, letter_type::LetterType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    pub location: Coordinate,
    pub char: char,
    pub letter_type: LetterType,
    pub generation: u32,
    pub render_letter: bool,
    pub color: Color,
}

impl Pixel {
    pub fn new(
        location: Coordinate,
        char: char,
        letter_type: LetterType,
        generation: u32,
        // <----appearence---->
        //if false background is rendered with specified color
        render_letter: bool,
        color: Color, //<----appearence---->
    ) -> Pixel {
        Pixel {
            location,
            char,
            letter_type,
            generation,
            render_letter,
            color,
        }
    }
}
