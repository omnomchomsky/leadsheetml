use clap::Parser;
use std::{fs};

use markup_engine::{HtmlEngine, MarkdownEngine};
use render::DefaultLeadSheetRenderer;
use crate::render::LeadSheetRenderer;
mod transpose;

mod parser;
mod ast;
mod render;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .lmpl file
    #[arg()]
    filename: String,

    /// Output format (default is markdown)
    #[arg(long, value_parser = ["markdown", "html"], default_value = "markdown")]
    format: String,

    #[arg(long, default_value_t = 0)]
    transpose: isize,

    #[arg(long, default_value_t = 0)]
    transpose_down: isize
}


fn main() {
    let args = Args::parse();

    if !args.filename.ends_with(".lmpl") {
        eprintln!("Invalid file extension: {}", args.filename);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args.filename).expect("Failed to read input file");
    let mut ast = parser::parse_song_from_str(&input);
    if args.transpose != 0 {
        ast = transpose::transpose_song(
            ast,
            args.transpose)
    }
    if args.transpose_down != 0 {
        ast = transpose::transpose_song(
            ast,
            -args.transpose_down)
    }
    match args.format.as_str() {
        "html" => {
            let html = DefaultLeadSheetRenderer.render_song(&HtmlEngine, &ast);
            println!("{}", html);
        }
        _ => {
            let md = DefaultLeadSheetRenderer.render_song(&MarkdownEngine, &ast);
            println!("{}", md);
        }
    }
}

