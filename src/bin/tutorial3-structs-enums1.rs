pub struct Xval(u32);
pub struct Yval(u32);

pub struct Coord {
    x: Xval,
    y: Yval,
}

pub type WebColorVal = String;

pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Cyan,
    Magenta,
    Black,
    White
}

pub enum ColorType {
    Rgb(u32, u32, u32),
    Hsb(u32, u32, u32),
    Web(WebColorVal),
    Standard(Color),
}

pub struct Pixel {
    coord: Coord,
    color: ColorType,
}

fn main() {
    let pix1 = Pixel {
        coord: Coord {
            x: Xval(320),
            y: Yval(200),
        },
        color: ColorType::Standard(Color::Black)
    };

    let color_str = match pix1.color {
        ColorType::Rgb(r, g, b) => format!("R: {}, G: {}, B: {}", r, g, b),
        ColorType::Hsb(h, s, b) => format!("H: {}, S: {}, B: {}", h, s, b),
        ColorType::Standard(color) => {
            match color {
                Color::Red => "RED".to_string(),
                Color::Green => "GREEN".to_string(),
                Color::Blue => "BLUE".to_string(),
                _ => "OTHER".to_string(),
            }
        },
        ColorType::Web(wcv) => wcv,
    };

    println!("Pixel created at {}x{} with color {}",
             pix1.coord.x.0,
             pix1.coord.y.0,
             color_str);
}
