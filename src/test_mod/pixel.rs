use super::color::{Color, ColorType};

pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        Coord {
            x,
            y
        }
    }
}

pub struct Pixel {
    pub coord: Coord,
    pub color: ColorType,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: ColorType) -> Self {
        Pixel {
            coord: Coord::new(x, y),
            color
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}x{}, color: {}",
                self.coord.x,
                self.coord.y,
                self.color.to_string())
    }
}
