use crate::map::map::Map;

pub trait Grow {
    fn grow(map: &mut Map);
}