use super::dimensions::Dimensions;
use super::map::Map;
use super::map_stats::MapStats;
use crate::services::animations::animation_traits::{Grow, Write};

pub struct MapManager {
    pub map: Map,
    map_stats: MapStats,
    animation_svc: Box<dyn Grow>,
}
impl Write for MapManager {}

impl MapManager {
    pub fn new(map: Map, svc: Box<dyn Grow + Send>) -> MapManager {
        //terminal dimension can be obtained through running termion::terminal_size()
        let terminal_dimensions = Dimensions {
            // if dimensions dont fit the screen the lines will overflow into the next one (graphic bug)
            width: 90,
            height: 50,
        };
        Self {
            map,
            map_stats: MapStats::new(),
            animation_svc: svc,
        }
    }

    pub fn run_animation(&mut self) {
        self.map_stats.generation += 1;
        self.animation_svc.grow(&mut self.map, &mut self.map_stats);
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
