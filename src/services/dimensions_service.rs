use crate::map::dimensions::Dimensions;

pub struct DimensionsService {}

impl DimensionsService {
    pub fn get_dimensions() -> Dimensions {
        // actual dimensions can be acquired with  termion::terminal_size()
        Dimensions {
            width: 90,
            height: 50,
        }
    }
}
