use crate::coordiante::Coordinate;
use crate::dimensions::Dimensions;
use crate::letter_type::LetterType;
use crate::pixel::Pixel;

pub struct Map {
    pub vec: Vec<Vec<Pixel>>,
}
impl Map {
    pub fn new(dimensions: Dimensions) -> Map {
        let mut vec: Vec<Vec<Pixel>> = vec![
            vec![
                Pixel::new(
                    Coordinate::new(0, 0),
                    ' ',
                    LetterType::Regular,
                    1000,
                    true,
                    colored::Color::White
                );
                dimensions.width as usize
            ];
            dimensions.height as usize
        ];

        //<---- assign coordinates ---->
        let mut i = 0;
        for column in vec.iter_mut() {
            let mut j = 0;
            for pixel in column.iter_mut() {
                pixel.location.x = i as u32;
                pixel.location.y = j as u32;
                j += 1;
            }
            i += 1;
        }
        //<---- assign coordinates ---->

        Map { vec: vec }
    }
}
