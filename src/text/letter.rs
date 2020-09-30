use std::{rc::Rc, path::PathBuf};
use image::*;

use super::{PageProps, images_map::ImagesMap, CHARS, paths};

#[derive(Debug)]
pub struct Letter<'a> {
    pub raw: char,
    width: Option<f32>,
    page_props: &'a PageProps<'a>,
    imgs_map: Rc<ImagesMap<'a>>,
}

impl<'a> Letter<'a> {
    pub fn new(letter : char, page_props : &'a PageProps<'a>, imgs_map : Rc<ImagesMap<'a>>) -> Letter<'a> {
        Letter { raw: letter, width: None, page_props, imgs_map }
    }

    pub fn char_name(letter : char) -> String {
        match letter {
            '?' => String::from("question_mark"),
            '!' => String::from("exclamation_mark"),
            ';' => String::from("semicolon"),
            ',' => String::from("comma"),
            '.' => String::from("dot"),
            ':' => String::from("colon"),
            _ => {
                if CHARS.contains(&letter.to_uppercase().next().unwrap()) {
                    let upper = match letter.is_uppercase() {
                        true => "uc",
                        false => "lc"
                    };

                    format!("{}-{}", letter.to_lowercase().next().unwrap(), upper)
                } else {
                    String::from("question_mark")
                }
            },
        }
    }

    pub fn get_letter_path(letter : char, dic_name: &Option<&str>) -> PathBuf {
        paths::letter_path(Letter::char_name(letter), dic_name)
    }

    pub fn get_img(letter : char, dic_name: &Option<&str>) -> DynamicImage {
        let path = Letter::get_letter_path(letter, dic_name);
        image::open(path).unwrap()
    }

    pub fn get_resized_image(letter : char, line_height : f32, dic_name: &Option<&str>) -> RgbaImage {
        let img = Letter::get_img(letter, dic_name);

        let prop = line_height / img.height() as f32;
        let width = img.width() as f32 * prop;
        let img = img.resize(width as u32, line_height as u32, imageops::FilterType::Lanczos3);
        let img = img.to_rgba();

        img
    }

    pub fn img(&mut self) -> Rc<RgbaImage> {
        match &self.imgs_map.get(self.raw) {
            Some(img_ref) => Rc::clone(img_ref),
            None => {
                &self.imgs_map.insert_letter(self.raw);
                self.img()
            }
        }
    }

    pub fn width(&mut self) -> f32 {
        match self.width {
            Some(width) => width,
            None => {
                self.width = Some(self.img().width() as f32);
                self.width.unwrap()
            }
        }
    }
}
