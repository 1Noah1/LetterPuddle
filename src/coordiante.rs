#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}
impl Coordinate {
    pub fn new(x: u16, y: u16) -> Coordinate {
        Coordinate { x: x, y: y }
    }
}
