#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}
impl Coordinate {
    pub fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x: x, y: y }
    }
}
