use crate::coordiante::Coordinate;

#[derive(Debug,Clone, Copy)]
pub struct Pixel {
    pub location: Coordinate,
    pub char: char,
}

impl Pixel {
    pub fn new(location: Coordinate, char: char) -> Pixel {
        Pixel {
            location: location,
            char: char,
        }
    }
}