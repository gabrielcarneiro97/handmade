mod words;
mod colors;
mod papers;
mod text;

use words::*;
use text::*;

fn main() {
    let word = Word::new("Teste");

    println!("{:#?}", papers::A4);
}
