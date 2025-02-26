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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        assert_eq!(Coordinate::new(10, 10), Coordinate { x: 10, y: 10 })
    }
}
