use crate::pixel::coordiante::Coordinate;

pub struct MapStats {
    pub generation: u32,
    pub last_written_pos: Vec<Coordinate>,
}

impl MapStats {
    pub fn new() -> MapStats {
        MapStats {
            generation: 0,
            last_written_pos: vec![],
        }
    }
}
