use std::fs;
use pest::Parser;

mod parser;
mod ast;
mod markdown;

fn main() {
    let input = fs::read_to_string("SongBook/Genesis/for_absent_friends.impl").unwrap();
    let ast = parser::parse_song_from_str(&input);
    println!("{:?}", ast);
    let md = markdown::render_song(&ast);
    println!("{}", md);
}

