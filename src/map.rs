use crate::coordiante::Coordinate;
use crate::dimensions::Dimensions;
use crate::letter_type::LetterType;
use crate::pixel::Pixel;

#[derive(PartialEq, Eq, Debug)]
pub struct Map {
    pub vec: Vec<Vec<Pixel>>,
}
impl Map {
    pub fn new(dimensions: Dimensions) -> Map {
        let mut vec: Vec<Vec<Pixel>> =
            vec![
                vec![
                    Pixel::new(Coordinate::new(0, 0), ' ', LetterType::Regular, 1000,);
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

    // return how many pixels are in a row (left to right)
    pub fn get_row_len(&self) -> usize {
        self.vec[0].len()
    }
    // returns amount of rows (up to down)
    pub fn get_column_len(&self) -> usize {
        self.vec.len()
    }
    pub fn get_pixel(&self, location: Coordinate) -> &Pixel {
        &self.vec[location.x as usize][location.y as usize]
    }
    fn get_mut_pixel(&mut self, location: Coordinate) -> &mut Pixel {
        &mut self.vec[location.x as usize][location.y as usize]
    }
    pub fn set_pixel(&mut self, new_pixel: Pixel) {
        self.vec[new_pixel.location.x as usize][new_pixel.location.y as usize] = new_pixel
    }
    pub fn is_border_pos(&self, location: Coordinate) -> bool {
        match self.get_pixel(location).letter_type {
            LetterType::Border => true,
            LetterType::Regular => false,
        }
    }
    pub fn add_to_border(&mut self, location: Coordinate) {
        self.get_mut_pixel(location).letter_type = LetterType::Border;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let dimensions = Dimensions {
            width: 90,
            height: 50,
        };
        let map = Map::new(dimensions);

        //<---- create empty 2Dvec ---->
        let mut vec: Vec<Vec<Pixel>> =
            vec![
                vec![
                    Pixel::new(Coordinate::new(0, 0), ' ', LetterType::Regular, 1000,);
                    dimensions.width as usize
                ];
                dimensions.height as usize
            ];
        //<---- create empty 2Dvec ---->

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

        assert_eq!(map, Map { vec: vec })
    }

    #[test]
    fn get_row_len() {
        let dimensions = Dimensions {
            width: 90,
            height: 50,
        };
        let map = Map::new(dimensions);

        assert_eq!(map.get_row_len(), map.vec[0].len());
    }

    #[test]
    fn get_column_len() {
        let dimensions = Dimensions {
            width: 90,
            height: 50,
        };
        let map = Map::new(dimensions);

        assert_eq!(map.get_column_len(), map.vec.len());
    }

    #[test]
    fn get_pixel() {
        let dimensions = Dimensions {
            width: 90,
            height: 50,
        };
        let map = Map::new(dimensions);

        assert_eq!(map.get_pixel(Coordinate { x: 10, y: 10 }), &map.vec[10][10]);
    }

    #[test]
    fn set_pixel() {
        let dimensions = Dimensions {
            width: 90,
            height: 50,
        };
        let mut map = Map::new(dimensions);

        let new_pixel_pos = Coordinate::new(15, 15);
        let pixel = Pixel::new(new_pixel_pos, 'A', LetterType::Regular, 1000);

        map.set_pixel(pixel);
        assert_eq!(map.get_pixel(new_pixel_pos), &pixel)
    }

    #[test]
    fn border_pos() {
        let mut map = Map::new(Dimensions::new(90, 50));

        let is_border_coord = Coordinate::new(15, 15);
        let is_not_border_coord = Coordinate::new(0, 0);

        map.add_to_border(is_border_coord);

        assert_eq!(map.is_border_pos(is_border_coord), true);
        assert_eq!(map.is_border_pos(is_not_border_coord), false);
    }
}
