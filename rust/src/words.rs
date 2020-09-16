
#[derive(Debug)]
struct Letter {
    raw: char,
    width: f32,
}

impl Letter {
    pub fn letter_width(letter: char) -> f32 {
        match letter {
            _ => 7.0,
        }
    }
}

#[derive(Debug)]
pub struct Word {
    raw: String,
    letters: Vec<Letter>,
    width: f32,
}

impl Word {
    pub fn new(str: &str) -> Word {
        let mut this = Word {
            raw: String::from(str),
            letters: Vec::new(),
            width: 0.0,
        };

        let chars : Vec<char> = this.raw.chars().collect();

        for letter in chars {
            let lw = Letter::letter_width(letter);
            this.letters.push(Letter { raw: letter, width: lw });
            this.width += lw;
        }

        this
    }
}

