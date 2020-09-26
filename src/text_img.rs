#[path = "./text/mod.rs"]
mod text;

use text::*;

use image;

use std::{convert::AsRef, path::Path};

pub static LETTERS_FOLDER : &str = "./src/assets/";
pub static LETTERS_EXT : &str = ".png";

pub fn update_images<P : AsRef<Path>>(path : P) {
    let mut img = image::open(path).unwrap();

    let mut x_pos = 0;

    for c in &CHARS {
        let this = image::imageops::crop(&mut img, 0 + x_pos, 0, 30, 35);

        let path = text::Letter::get_letter_path(*c);

        this.to_image().save(path).unwrap();

        let path = text::Letter::get_letter_path(c.to_lowercase().next().unwrap());

        this.to_image().save(path).unwrap();

        x_pos += 30;
    }
}
