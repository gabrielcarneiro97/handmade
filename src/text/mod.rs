pub mod words;

pub mod pages;

use words::{Word, Letter};
use pages::PageProps;

use image::*;

pub struct Line {
    words: Vec<Word>,
    width: f32,
    spaces_counter: i32,
}

impl Line {
    pub fn new() -> Line {
        Line {
            words: Vec::new(),
            width: 0.0,
            spaces_counter: 0,
        }
    }

    pub fn push(&mut self, word: Word) {
        if self.width != 0.0 {
            self.spaces_counter += 1;
            self.width += Letter::spc_width();
        }
        self.width += word.width;
        self.words.push(word);
    }
}

pub struct Text<'a> {
    raw: String,
    lines: Vec<Line>,
    page_props: &'a PageProps<'a>,
}

impl Text<'_> {
    pub fn new<'a>(page_props: &'a PageProps) -> Text<'a> {
        Text {
            raw: String::new(),
            page_props,
            lines: Vec::new(),
        }
    }

    pub fn push_word(&mut self, word: Word) {
        match self.lines.last_mut() {
            Some(actual_line) => {
                if actual_line.width + word.width + Letter::spc_width() > self.page_props.line_max_width() {
                    self.lines.push(Line::new());
                    self.push_word(word);
                } else {
                    actual_line.push(word);
                }
            },
            None => {
                self.lines.push(Line::new());
                self.push_word(word);
            }
        }
    }

    pub fn parse(&mut self, string: String) {
        for s_word in string.split(' ') {
            self.push_word(Word::new(s_word));
        }

        self.raw = string;
    }

    pub fn parse_str(&mut self, str: &str) {
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

    pub fn to_img(&self) -> Vec<RgbImage> {
        let mut pages = Vec::new();

        let mut page = self.page_props.white_page();

        let mut y = self.page_props.margins;
        for line in &self.lines {
            let mut x = self.page_props.margins;
            for word in &line.words {
                for letter in &word.letters {
                    let l_img = Letter::get_img(letter.raw).unwrap();
                    imageops::overlay(&mut page, &l_img.to_rgb(), x as u32, y as u32);
                    x += letter.width;
                }

                x += Letter::spc_width();
            }

            y += self.page_props.line_height;

            if y >= self.page_props.canvas.height as f32 - self.page_props.margins + self.page_props.line_height {
                pages.push(page);
                page = self.page_props.white_page();
                y = self.page_props.margins;
            }

        }

        pages.push(page);
        pages
    }
}
