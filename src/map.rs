use std::collections::HashMap;

use crate::coordiante::Coordinate;
use crate::dimensions::Dimensions;
use crate::letter_type::LetterType;
use crate::pixel::Pixel;

pub struct Map {
    pub vec: Vec<Vec<Pixel>>,
    border_pos: HashMap<Coordinate, LetterType>,
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

        Map {
            vec: vec,
            border_pos: HashMap::new(),
        }
    }
    // return how many pixels are in a row (left to right)
    pub fn get_row_len(&self) -> usize {
        self.vec[0].len()
    }
    // returns amount of rows (up to down)
    pub fn get_column_len(&self) -> usize {
        self.vec.len()
    }
    pub fn get_pixel(&self, location: Coordinate) -> Pixel {
        self.vec[location.x as usize][location.y as usize]
    }
    pub fn set_pixel(&mut self, new_pixel: Pixel) {
        self.vec[new_pixel.location.x as usize][new_pixel.location.y as usize] = new_pixel
    }
    pub fn is_border_pos(&self, location: Coordinate) -> bool {
        match self.border_pos.get(&location) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn add_to_border(&mut self, location: Coordinate) {
        self.border_pos.insert(location, LetterType::Border);
    }
}
