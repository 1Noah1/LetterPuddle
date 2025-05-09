use core::panic;

use crate::map::{map::Map, map_stats::MapStats};

use super::animation_traits::{Grow, Write};

pub struct Alphabet {}

impl Write for Alphabet {}

impl Alphabet {
    pub fn new() -> Alphabet {
        Alphabet {}
    }
}

impl Grow for Alphabet {
    fn grow(&self, map: &mut Map, map_stats: &mut MapStats) {
        panic!("this animation is not implemented yet")
    }
}
