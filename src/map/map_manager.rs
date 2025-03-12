use std::clone;
use std::time::Instant;

use crate::config::general_config::GeneralConfig;
use crate::pixel::coordiante::Coordinate;
use crate::pixel::letter_type::LetterType;
use crate::pixel::pixel::Pixel;
use crate::render::terminal_render_engine::TerminalRenderEngine;
use crate::services::animations::grow::{self, Grow};
use crate::services::letter_service::LetterService;

use super::dimensions::Dimensions;
use super::map::Map;

pub struct MapManager {
    pub map: Map,
    // is read by map::new()
    //terminal_dimensions: Dimensions,
    last_written_pos: Vec<Coordinate>,
    generation: u32,
    config: GeneralConfig,
}

impl MapManager {
    pub fn new(config: &GeneralConfig, svc: impl Grow) -> MapManager {
        //terminal dimension can be obtained through running termion::terminal_size()
        let terminal_dimensions = Dimensions {
            // if dimensions dont fit the screen the lines will overflow into the next one (graphic bug)
            width: 90,
            height: 50,
        };
        Self {
            last_written_pos: vec![],
            //terminal_dimensions: terminal_dimensions,
            map: Map::new(terminal_dimensions),
            generation: 0,
            config: config.clone(),

        }
    }

    pub fn run(&mut self) {
        self.init();
        self.write_borders();
        
        let mut i = 0;
        let mut engine = TerminalRenderEngine::new(config.render_config,);
        
        let start = Instant::now();
        
        loop {
            let start = Instant::now();
            engine.render();
            // TerminalRenderEngine::render(&config.render_config, &mut manager.map);
        
            self.writer(self.svc.grow());
            self.grow();
            thread::sleep(time::Duration::from_millis(50));
            engine.add_frame(manager.map.clone());
        
            let end = Instant::now();
            println!("time: {:?}", end.duration_since(start));
            if i == 68 {
                break;
            }
            i += 1;
        }

    }

    fn init(&mut self) {
        self.write_borders();
    }

    fn write_borders(&mut self) {
        //tried reducing lines by looping more
        // i think i barely improved it lol

        let right_left = '|';
        let top_bottom = '-';
        //top and bottom border
        let mut j = 0;
        while j < 2 {
            let mut i = 0;
            while i <= self.map.get_row_len() - 1 {
                let x;
                let y;
                if j < 1 {
                    x = 0;
                    y = i;
                } else {
                    x = self.map.get_column_len() - 1;
                    y = i;
                }
                self.writer(Pixel::new(
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
            while i <= self.map.get_column_len() - 1 {
                let x;
                let y;
                if j < 1 {
                    x = i;
                    y = 0;
                } else {
                    x = i;
                    y = self.map.get_row_len() - 1;
                }
                self.writer(Pixel::new(
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
    
    

    fn writer(&mut self, pixel: Pixel) {
        self.map.set_pixel(pixel);
        match pixel.letter_type {
            LetterType::Border => {
                self.map.add_to_border(pixel.location);
            }
            LetterType::Regular => self.last_written_pos.push(pixel.location),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::general_config;
    // rewrite after refactor
/* 
    #[test]
    fn grow() {
        let config = GeneralConfig::new(true, true, true);
        let mut map_manager = MapManager::new(&config);
        let dimensions = Dimensions::new(90, 50);

        map_manager.init();

        let middle_letter_pos = Coordinate::new((dimensions.height) / 2, (dimensions.width) / 2);

        if map_manager.map.get_pixel(middle_letter_pos).char == 'A' {
            println!("correct_middle letter");
        } else {
            panic!("middle_letter is not 'A'")
        }

        let mut i = 0;
        while i <= 40 {
            map_manager.grow();
            i += 1;
        }
        for (mut i, letter) in ('A' as u8..='Z' as u8).enumerate() {
            //initial offset
            i += 1;
            let letter_in_map = map_manager.map.get_pixel(Coordinate::new(
                middle_letter_pos.x,
                middle_letter_pos.y - i as u32,
            ));
            if letter_in_map.char != letter as char {
                panic!("iterative letter map is broken")
            }
        }
    }
*/
}
