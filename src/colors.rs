#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32,
}

pub static BLUE : Color = Color { red: 0, green: 0, blue: 255, alpha: 1.0 };
pub static BLACK : Color = Color { red: 0, green: 0, blue: 0, alpha: 1.0 };
