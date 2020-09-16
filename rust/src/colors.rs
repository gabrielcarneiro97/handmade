#[derive(Debug)]
pub struct Color {
    red: i16,
    green: i16,
    blue: i16,
    alpha: f32,
}

pub static BLUE : Color = Color { red: 0, green: 0, blue: 255, alpha: 1.0 };
pub static BLACK : Color = Color { red: 0, green: 0, blue: 0, alpha: 1.0 };
