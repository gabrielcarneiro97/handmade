#[path = "./text/mod.rs"]
mod text;

use text::*;

use image;

use std::{convert::AsRef, path::Path};

pub static LETTERS_FOLDER : &str = "./src/assets/";
pub static LETTERS_EXT : &str = ".png";

pub fn update_images<P: AsRef<Path>>(path: P) {
    let mut img = match image::open(path) {
        Ok(i) => i,
        Err(e) => panic!(e),
    };

    let mut x_pos = 0;

    for c in &CHARS {
        let this = image::imageops::crop(&mut img, 0 + x_pos, 0, 30, 35);

        let path = format!("{}{}{}", LETTERS_FOLDER, Letter::char_name(*c), LETTERS_EXT);

        this.to_image().save(path).unwrap();

        x_pos += 30;
    }
}
