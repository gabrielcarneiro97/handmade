use std::rc::Rc;
use image::*;

pub mod pages;
pub mod paths;
pub mod images_map;
pub mod letter;
pub mod word;
pub mod line;

pub use pages::PageProps;
pub use images_map::ImagesMap;
pub use letter::Letter;
pub use word::Word;
pub use line::Line;


pub static CHARS_ONLY_UPPER : [char; 32] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '?', '!', ',', '.', ';', ':'];

pub static CHARS_UNDER : [char; 6] = ['g', 'j', 'p', 'q', 'y', ','];

pub static CHARS : [char; 58] = ['A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', '?', '!', ',', '.', ';', ':'];

pub struct Text<'a> {
    raw: String,
    lines: Vec<Line<'a>>,
    page_props: &'a PageProps<'a>,
    imgs_map: Rc<ImagesMap<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(page_props : &'a PageProps) -> Text<'a> {
        Text {
            raw: String::new(),
            page_props,
            lines: Vec::new(),
            imgs_map: Rc::new(ImagesMap::new(page_props))
        }
    }

    pub fn new_with_map(page_props : &'a PageProps, imgs_map : Rc<ImagesMap<'a>>) -> Text<'a> {
        Text {
            raw: String::new(),
            page_props,
            lines: Vec::new(),
            imgs_map,
        }
    }

    pub fn new_with_dic_name(page_props : &'a PageProps, dic_name: &'a str) -> Text<'a> {
        Text {
            raw: String::new(),
            page_props,
            lines: Vec::new(),
            imgs_map: Rc::new(ImagesMap::new_with_dic_name(page_props, dic_name))
        }
    }

    pub fn push_word(&mut self, s_word : &str) {
        match self.lines.last_mut() {
            Some(actual_line) => {
                let word = Word::new(s_word, self.page_props, Rc::clone(&self.imgs_map));

                if actual_line.width + word.width + self.page_props.space_width > self.page_props.line_max_width() {
                    self.lines.push(Line::new(self.page_props, Rc::clone(&self.imgs_map)));
                    self.push_word(s_word);
                } else {
                    actual_line.push(word);
                }
            },
            None => {
                self.lines.push(Line::new(self.page_props, Rc::clone(&self.imgs_map)));
                self.push_word(s_word);
            }
        }
    }

    pub fn imgs_map(&self) -> &ImagesMap {
        &self.imgs_map
    }

    pub fn parse(&mut self, string: String) {
        for s_word in string.split(' ') {
            self.push_word(s_word);
        }

        self.raw = string;
    }

    pub fn parse_str(&mut self, str : &str) {
        self.parse(String::from(str));
    }

    pub fn print(&self) {
        for line in &self.lines {
            for word in &line.words {
                print!("{} ", word.get_raw());
            }
            println!();
        }
    }

    pub fn to_img(&mut self) -> Vec<RgbaImage> {
        let mut pages = Vec::new();

        let mut page = self.page_props.white_page();

        let mut y = self.page_props.margins;
        for line in self.lines.iter_mut() {
            let mut x = self.page_props.margins;
            for word in line.words.iter_mut() {
                for letter in word.letters.iter_mut() {
                    let l_img = &*letter.img();
                    imageops::overlay(&mut page, l_img, x as u32, y as u32);
                    x += letter.width();
                }

                x += self.page_props.space_width;
            }

            y += self.page_props.line_height;

            if y + self.page_props.line_height >= self.page_props.canvas.height as f32 - self.page_props.margins {
                pages.push(page);
                page = self.page_props.white_page();
                y = self.page_props.margins;
            }

        }

        pages.push(page);
        pages
    }

    pub fn to_files(&mut self, files_name : Option<&str>) {
        let images = self.to_img();

        match std::fs::create_dir(paths::output_dir()) {
            Ok(_) => (),
            Err(_) => ()
        };

        for (i, img) in images.iter().enumerate() {
            img.save(paths::page_path(i + 1, files_name)).unwrap();
        }
    }

    pub fn parse_to_png(&mut self, str : &str, files_name : Option<&str>) {
        self.parse_str(str);
        self.to_files(files_name);
    }
}
