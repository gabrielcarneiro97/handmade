pub mod pages;

use pages::PageProps;

use image::*;

use std::{cell::RefCell, collections::HashMap};

pub static CHARS : [char; 32] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '?', '!', ',', '.', ';', ':'];

#[derive(Debug)]
pub struct Letter<'a> {
    pub raw: char,
    width: f32,
    img: Option<image::RgbaImage>,
    img_ref: Option<&'a image::RgbaImage>,
    page_props: &'a PageProps<'a>,
    imgs_map: Option<&'a RefCell<HashMap<String, image::RgbaImage>>>,
}

impl<'a> Letter<'a> {
    pub fn new(letter: char, page_props: &'a PageProps<'a>) -> Letter<'a> {
        let mut this = Letter { raw: letter, width: 0.0, img: None, page_props, imgs_map: None, img_ref: None };
        this.set_image();
        this
    }

    pub fn char_name(letter: char) -> String {
        match letter {
            '?' => String::from("question_mark"),
            '!' => String::from("exclamation_mark"),
            ';' => String::from("semicolon"),
            ',' => String::from("comma"),
            '.' => String::from("dot"),
            ':' => String::from("colon"),
            _ => format!("{}", letter),
        }
    }

    pub fn get_letter_path(letter: char) -> String {
        format!("./src/assets/{}.png", Letter::char_name(letter))
    }

    pub fn get_img(letter: char) -> DynamicImage {
        let path = Letter::get_letter_path(letter);

        image::open(path).unwrap()
    }

    pub fn set_image(&mut self) {
        match self.imgs_map {
            Some(imgs_map) => {
                let key = Letter::get_letter_path(self.raw);
                match imgs_map.borrow().get(&key) {
                    Some(img) => {
                        self.img_ref = Some(img);
                    }
                    None => {}
                };
            },
            None => {
                let img = Letter::get_img(self.raw);

                let height = self.page_props.line_height;
                let prop = height / img.height() as f32;
                let width = img.width() as f32 * prop;
                let img = img.resize(width as u32, height as u32, image::imageops::FilterType::Lanczos3);
                let img = img.to_rgba();

                self.img = Some(img);
            },
        };
    }

    pub fn img(&mut self) -> &image::RgbaImage {
        match &self.img_ref {
            Some(i) => i,
            None => match &self.img {
                Some(i) => &i,
                None => panic!("img error!"),
            }
        }
    }

    pub fn width(&mut self) -> f32 {
        if self.width != 0.0 {
            self.width
        } else {
            match &self.img {
                Some(img) => {
                    self.width = img.width() as f32;
                    self.width
                },
                None => {
                    self.set_image();
                    self.width = self.img().width() as f32;
                    self.width
                },
            }
        }
    }
}

#[derive(Debug)]
pub struct Word<'a> {
    pub raw: String,
    pub letters: Vec<Letter<'a>>,
    pub width: f32,
    page_props: &'a pages::PageProps<'a>
}

impl<'a> Word<'a> {
    pub fn new(str: &str, page_props: &'a pages::PageProps<'a>) -> Word<'a> {
        let mut this = Word {
            raw: String::from(str),
            letters: Vec::new(),
            width: 0.0,
            page_props,
        };

        let chars : Vec<char> = this.raw.chars().collect();

        for l_char in chars {
            let mut letter = Letter::new(l_char, this.page_props);
            this.width += letter.width();
            this.letters.push(letter);
        }

        this
    }

    pub fn get_raw(&self) -> &String {
        &self.raw
    }
}

pub struct Line<'a> {
    words: Vec<Word<'a>>,
    width: f32,
    spaces_counter: i32,
    page_props: &'a pages::PageProps<'a>
}

impl<'a> Line<'a> {
    pub fn new(page_props: &'a pages::PageProps<'a>) -> Line<'a> {
        Line {
            words: Vec::new(),
            width: 0.0,
            spaces_counter: 0,
            page_props,
        }
    }

    pub fn push(&mut self, word: Word<'a>) {
        if self.width != 0.0 {
            self.spaces_counter += 1;
            self.width += word.page_props.space_width;
        }
        self.width += word.width;
        self.words.push(word);
    }
}

pub struct Text<'a> {
    raw: String,
    lines: Vec<Line<'a>>,
    page_props: &'a PageProps<'a>,
}

impl<'a> Text<'a> {
    pub fn new(page_props: &'a PageProps) -> Text<'a> {
        Text {
            raw: String::new(),
            page_props,
            lines: Vec::new(),
        }
    }

    pub fn push_word(&mut self, word: Word<'a>) {
        match self.lines.last_mut() {
            Some(actual_line) => {
                if actual_line.width + word.width + self.page_props.space_width > self.page_props.line_max_width() {
                    self.lines.push(Line::new(self.page_props));
                    self.push_word(word);
                } else {
                    actual_line.push(word);
                }
            },
            None => {
                self.lines.push(Line::new(self.page_props));
                self.push_word(word);
            }
        }
    }

    pub fn parse(&mut self, string: String) {
        for s_word in string.split(' ') {
            self.push_word(Word::new(s_word, self.page_props));
        }

        self.raw = string;
    }

    pub fn parse_str(&mut self, str: &'a str) {
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
                    let l_img = letter.img();
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
}
