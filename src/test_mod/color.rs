pub type WebColorVal = String;

#[derive(Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Cyan,
    Magenta,
    Black,
    White,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Red => "RED".to_string(),
            Color::Green => "GREEN".to_string(),
            Color::Blue => "BLUE".to_string(),
            _ => "OTHER".to_string(),
        }
    }
}

pub enum ColorType {
    Rgb(u32, u32, u32),
    Hsb(u32, u32, u32),
    Web(WebColorVal),
    Standard(Color)
}

impl ColorType {
    pub fn to_string(&self) -> String {
        match self {
            ColorType::Rgb(r, g, b) => format!("R: {}, G: {}, B: {}", r, g, b),
            ColorType::Hsb(h, s, b) => format!("Hue: {}, Sat: {}, Br: {}", h, s, b),
            ColorType::Standard(color) => color.to_string(),
            // Example of Compiler error:
            // ColorType::Web(wcv) => wcv,
            // Since "&self" is a reference, this is returning a reference to the string.
            // Thus, we have to clone it to make a new string (as the other match arms are
            // returning actual string values).
            ColorType::Web(wcv) => wcv.clone(),
        }
    }
}

impl Clone for ColorType {
    fn clone(&self) -> Self {
        match *self {
            ColorType::Rgb(r, g, b) => ColorType::Rgb(r, g, b),
            ColorType::Hsb(h, s, b) => ColorType::Hsb(h, s, b),
            // Example of Compiler error:
            // ColorType::Web(wcv) => ColorType::Web(wcv),
            // This is trying to "steal" the original "self" object's value.  Numerics, like
            // the above patterns, have a "Copy" trait, which allows them to automatically
            // copy when "stolen", but Strings so not have this, so we have to use a "clone()"
            // on it (much like how we are defining a "clone" right now).
            ColorType::Web(ref wcv) => ColorType::Web(wcv.clone()),
            // Similar to above.
            ColorType::Standard(ref color) => ColorType::Standard(color.clone()),
        }
    }
}
