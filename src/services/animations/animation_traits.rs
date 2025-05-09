use crate::{
    map::{map::Map, map_stats::MapStats},
    pixel::{coordiante::Coordinate, letter_type::LetterType, pixel::Pixel},
};
pub trait Grow{
    fn grow(&self, map: &mut Map, map_stats: &mut MapStats) ;
}
pub trait Write {
    fn write(&self, map: &mut Map, last_written_pos: Option<&mut Vec<Coordinate>>, pixel: Pixel) {
        map.set_pixel(pixel);
        match pixel.letter_type {
            LetterType::Border => {
                map.add_to_border(pixel.location);
            }
            LetterType::Regular => 
                if let Some(last_written) = last_written_pos {
                    last_written.push(pixel.location);
            }
        }
    }
    fn write_borders(&self, map: &mut Map) {
        //tried reducing lines by looping more
        // i think i barely improved it lol

        let right_left = '|';
        let top_bottom = '-';
        //top and bottom border
        let mut j = 0;
        while j < 2 {
            let mut i = 0;
            while i <= map.get_row_len() - 1 {
                let x;
                let y;
                if j < 1 {
                    x = 0;
                    y = i;
                } else {
                    x = map.get_column_len() - 1;
                    y = i;
                }
                self.write(map, None,Pixel::new(
                    Coordinate::new(x as u32, y as u32),
                    top_bottom,
                    LetterType::Border,
                    0,
                ));
                i += 1;
            }
            j += 1;
        }

        //left and right border
        let mut j = 0;
        while j < 2 {
            let mut i = 0;
            while i <= map.get_column_len() - 1 {
                let x;
                let y;
                if j < 1 {
                    x = i;
                    y = 0;
                } else {
                    x = i;
                    y = map.get_row_len() - 1;
                }
                self.write(map, None,Pixel::new(
                    Coordinate::new(x as u32, y as u32),
                    right_left,
                    LetterType::Border,
                    0,
                ));
                i += 1;
            }
            j += 1;
        }
    }
}


