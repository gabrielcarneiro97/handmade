pub mod pages;

use pages::PageProps;

use image::*;

use std::{rc::Rc, cell::{RefCell}, collections::HashMap};

pub static CHARS : [char; 32] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '?', '!', ',', '.', ';', ':'];

#[derive(Debug)]
pub struct ImagesMap<'a> {
    map: RefCell<HashMap<String, Rc<RgbaImage>>>,
    page_props: &'a PageProps<'a>,
}

impl<'a> ImagesMap<'a> {
    pub fn new(page_props: &'a PageProps<'a>) -> ImagesMap {
        ImagesMap {
            map: RefCell::new(HashMap::new()),
            page_props
        }
    }

    pub fn keygen(letter: char) -> String {
        Letter::get_letter_path(letter)
    }

    pub fn insert_letter(&self, letter: char) {
        let key = ImagesMap::keygen(letter);
        let image = Letter::get_resized_image(letter, self.page_props.line_height);
        self.map.borrow_mut().insert(key, Rc::new(image));
    }

    pub fn populate(&self) {
        for letter in &CHARS {
            self.insert_letter(*letter);
            self.insert_letter(letter.to_lowercase().next().unwrap());
        }
    }

    pub fn get(&self, letter: char) -> Option<Rc<RgbaImage>> {
        let key = ImagesMap::keygen(letter);
        self.map.borrow().get(&key).map(|i| Rc::clone(i))
    }
}

#[derive(Debug)]
pub struct Letter<'a> {
    pub raw: char,
    width: Option<f32>,
    page_props: &'a PageProps<'a>,
    imgs_map: Rc<ImagesMap<'a>>,
}

impl<'a> Letter<'a> {
    pub fn new(letter: char, page_props: &'a PageProps<'a>, imgs_map: Rc<ImagesMap<'a>>) -> Letter<'a> {
        Letter { raw: letter, width: None, page_props, imgs_map }
    }

    pub fn char_name(letter: char) -> String {
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

    pub fn get_letter_path(letter: char) -> String {
        let folder : &str = "./src/assets/";
        let ext : &str = ".png";

        format!("{}{}{}", folder, Letter::char_name(letter), ext)
    }

    pub fn get_img(letter: char) -> DynamicImage {
        let path = Letter::get_letter_path(letter);

        image::open(path).unwrap()
    }

    pub fn get_resized_image(letter: char, line_height: f32) -> RgbaImage {
        let img = Letter::get_img(letter);

        let prop = line_height / img.height() as f32;
        let width = img.width() as f32 * prop;
        let img = img.resize(width as u32, line_height as u32, image::imageops::FilterType::Lanczos3);
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

#[derive(Debug)]
pub struct Word<'a> {
    pub raw: String,
    pub letters: Vec<Letter<'a>>,
    pub width: f32,
    page_props: &'a pages::PageProps<'a>,
    imgs_map: Rc<ImagesMap<'a>>,
}

impl<'a> Word<'a> {
    pub fn new(str: &str, page_props: &'a pages::PageProps<'a>, imgs_map: Rc<ImagesMap<'a>>) -> Word<'a> {
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

pub struct Line<'a> {
    words: Vec<Word<'a>>,
    width: f32,
    spaces_counter: i32,
    page_props: &'a pages::PageProps<'a>,
    imgs_map: Rc<ImagesMap<'a>>
}

impl<'a> Line<'a> {
    pub fn new(page_props: &'a pages::PageProps<'a>, imgs_map: Rc<ImagesMap<'a>>) -> Line<'a> {
        Line {
            words: Vec::new(),
            width: 0.0,
            spaces_counter: 0,
            page_props,
            imgs_map
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
    imgs_map: Rc<ImagesMap<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(page_props: &'a PageProps) -> Text<'a> {
        Text {
            raw: String::new(),
            page_props,
            lines: Vec::new(),
            imgs_map: Rc::new(ImagesMap::new(page_props))
        }
    }

    pub fn push_word(&mut self, s_word: &str) {
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
}
