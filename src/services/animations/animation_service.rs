use crate::{config::animation::Animation, map::map::Map};

use super::{grow::Grow, islands, snakes, speed::Speed};

pub struct AnimationService<'a> {
    map: &'a mut Map,
    speed: Speed
}
impl<'a> AnimationService <'a>{
    pub fn new(map: &'a mut Map, speed: Speed) -> AnimationService<'a>{
        AnimationService {
            map,
            speed
        }
    }

    pub fn alphabet(self){}
    pub fn islands(self){}
    pub fn snakes(self){}
}
impl<'a> Grow for AnimationService<'a> {
    fn grow(mut self, animation: Animation) -> bool{
        match animation {
            Animation::Alphabet => self.alphabet(),
            Animation::Islands => self.islands(),
            Animation::Snakes => self.snakes()
        }
        true

    }
}

