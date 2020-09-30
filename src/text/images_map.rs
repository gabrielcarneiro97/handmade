use std::{rc::Rc, cell::{RefCell}, collections::HashMap};
use image::RgbaImage;

use super::{Letter, PageProps, CHARS};


#[derive(Debug)]
pub struct ImagesMap<'a> {
    map: RefCell<HashMap<String, Rc<RgbaImage>>>,
    page_props: &'a PageProps<'a>,
    dic_name: Option<&'a str>
}

impl<'a> ImagesMap<'a> {
    pub fn new(page_props: &'a PageProps<'a>) -> ImagesMap {
        ImagesMap {
            map: RefCell::new(HashMap::new()),
            page_props,
            dic_name: None
        }
    }

    pub fn new_with_dic_name(page_props: &'a PageProps<'a>, dic_name: &'a str) -> ImagesMap<'a> {
        ImagesMap {
            map: RefCell::new(HashMap::new()),
            page_props,
            dic_name: Some(dic_name)
        }
    }

    pub fn keygen(letter : char) -> String {
        Letter::char_name(letter)
    }

    pub fn insert_letter(&self, letter : char) {
        let key = ImagesMap::keygen(letter);
        let image = Letter::get_resized_image(letter, self.page_props.line_height, &self.dic_name);
        self.map.borrow_mut().insert(key, Rc::new(image));
    }

    pub fn populate(&self) {
        for letter in CHARS.iter() {
            self.insert_letter(*letter);
            self.insert_letter(letter.to_lowercase().next().unwrap());
        }
    }

    pub fn get(&self, letter : char) -> Option<Rc<RgbaImage>> {
        let key = ImagesMap::keygen(letter);
        self.map.borrow().get(&key).map(|i| Rc::clone(i))
    }
}
