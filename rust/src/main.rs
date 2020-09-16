mod words;
mod colors;
mod pages;

use words::*;

fn main() {
    let word = Word::new("Teste");

    println!("{:#?}", pages::A4);
}
