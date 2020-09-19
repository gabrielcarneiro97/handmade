use image::{ImageResult, DynamicImage};

pub static CHARS : [char; 32] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '?', '!', ',', '.', ';', ':'];

#[derive(Debug)]
pub struct Letter {
    pub raw: char,
    pub width: f32,
}

impl Letter {
    pub fn new(letter: char) -> Letter {
        Letter { raw: letter, width: Letter::char_width(letter) }
    }

    pub fn char_name<'a>(letter: char) -> String {
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
pub struct Word {
    pub raw: String,
    pub letters: Vec<Letter>,
    pub width: f32,
}

impl Word {
    pub fn new(str: &str) -> Word {
        let mut this = Word {
            raw: String::from(str),
            letters: Vec::new(),
            width: 0.0,
        };

        let chars : Vec<char> = this.raw.chars().collect();

        for l_char in chars {
            let letter = Letter::new(l_char);
            this.width += letter.width;
            this.letters.push(letter);
        }

        this
    }

    pub fn get_raw(&self) -> &String {
        &self.raw
    }
}

