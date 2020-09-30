use std::rc::Rc;

use super::{Letter, PageProps, ImagesMap};

#[derive(Debug)]
pub struct Word<'a> {
    pub raw: String,
    pub letters: Vec<Letter<'a>>,
    pub width: f32,
    pub page_props: &'a PageProps<'a>,
    imgs_map: Rc<ImagesMap<'a>>,
}

impl<'a> Word<'a> {
    pub fn new(str : &str, page_props : &'a PageProps<'a>, imgs_map : Rc<ImagesMap<'a>>) -> Word<'a> {
        let chars : Vec<char> = str.chars().collect();
        let mut width = 0.0;
        let mut letters = Vec::new();

        for l_char in chars {
            let mut letter = Letter::new(l_char, page_props, Rc::clone(&imgs_map));
            width += letter.width();
            letters.push(letter);
        }

        Word {
            raw: String::from(str),
            letters,
            width,
            page_props,
            imgs_map
        }
    }

    pub fn get_raw(&self) -> &String {
        &self.raw
    }
}
