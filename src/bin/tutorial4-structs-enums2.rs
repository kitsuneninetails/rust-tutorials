use rust_tutorials::test_mod::{color::{Color, ColorType}, pixel::Pixel};

fn main() {
    let pix1 = Pixel::new(320, 200, ColorType::Standard(Color::Red));
    println!("New pixel created: {}", pix1.to_string());
}
