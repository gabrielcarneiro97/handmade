use std::rc::Rc;

use super::{Word, PageProps, ImagesMap};

pub struct Line<'a> {
    pub words: Vec<Word<'a>>,
    pub width: f32,
    spaces_counter: i32,
    page_props: &'a PageProps<'a>,
    imgs_map: Rc<ImagesMap<'a>>
}

impl<'a> Line<'a> {
    pub fn new(page_props : &'a PageProps<'a>, imgs_map : Rc<ImagesMap<'a>>) -> Line<'a> {
        Line {
            words: Vec::new(),
            width: 0.0,
            spaces_counter: 0,
            page_props,
            imgs_map
        }
    }

    pub fn push(&mut self, word : Word<'a>) {
        if self.width != 0.0 {
            self.spaces_counter += 1;
            self.width += word.page_props.space_width;
        }
        self.width += word.width;
        self.words.push(word);
    }
}
