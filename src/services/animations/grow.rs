use crate::{config::animation::Animation, map::map::Map};

use super::speed::Speed;


pub trait Grow {
    // return true if frame has been calculated
    fn grow(&mut self) -> bool{true}
}