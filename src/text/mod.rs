pub mod pages;

use pages::PageProps;

use image::*;

pub static CHARS : [char; 32] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '?', '!', ',', '.', ';', ':'];

#[derive(Debug)]
pub struct Letter<'a> {
    pub raw: char,
    pub width: f32,
    pub img: Option<image::RgbaImage>,
    page_props: &'a PageProps<'a>,
}

impl<'a> Letter<'a> {
    pub fn new(letter: char, page_props: &'a PageProps<'a>) -> Letter<'a> {
        Letter { raw: letter, width: Letter::char_width(letter), img: None, page_props }
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

    pub fn char_width(letter: char) -> f32 {
        match letter {
            _ => 30.0,
        }
    }

    pub fn spc_width() -> f32 {
        Letter::char_width(' ')
    }

    pub fn get_letter_path(letter: char) -> String {
        format!("./src/assets/{}.png", Letter::char_name(letter))
    }

    pub fn get_img(letter: char) -> ImageResult<DynamicImage> {
        let path = Letter::get_letter_path(letter);

        image::open(path)
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
            let letter = Letter::new(l_char, this.page_props);
            this.width += letter.width;
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
}

impl<'a> Line<'a> {
    pub fn new() -> Line<'a> {
        Line {
            words: Vec::new(),
            width: 0.0,
            spaces_counter: 0,
        }
    }

    pub fn push(&mut self, word: Word<'a>) {
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
