use pest::Parser;
use pest::error::Error;
use pest_derive::Parser;
use std::fs;
use std::io::read_to_string;
use crate::Rule::song;
use crate::Segment::{Inline, Measure};

fn main() {
    println!("Hello, world!");
}

#[derive(Parser)]
#[grammar = "leadsheetml.pest"]
struct LeadSheetMLParser;

pub struct Song {
    pub directives: Vec<Directive>,
    pub blocks: Vec<Block>,
}

pub struct Directive {
    pub name: String,
    pub value: String,
}

pub struct Block {
    pub section_name: String,
    pub lines: Vec<LyricLine>,
}

pub struct LyricLine {
    pub segments: Vec<Segment>,
}

pub enum Segment {
    Measure(Vec<ChordOrText>),
    Inline(Vec<ChordOrText>),
}

pub enum ChordOrText {
    Chord(Chord),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Chord {
    pub root: Note,
    pub quality: Option<String>,     // "maj", "min", "dim", etc.
    pub extensions: Vec<Option<String>>,   // "7", "9", "b5", etc.
    pub bass: Option<Note>,          // For slash chords
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub letter: NoteLetter,          // A-G
    pub accidental: Accidental,      // Sharp, Flat, Natural
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteLetter {
    A, B, C, D, E, F, G,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Accidental {
    Sharp,
    Flat,
    Natural,
    None,
}

fn parse(input: &str) -> Song {
    let unparsed_song = LeadSheetMLParser::parse(Rule::song, input).unwrap().next();
    let parsed_song = parse_song(unparsed_song.unwrap());
    parsed_song
}

fn parse_song(unparsed_song: pest::iterators::Pair<Rule>) -> Song {
    let mut directives:Vec<Directive> = Vec::new();
    let mut blocks:Vec<Block> = Vec::new();
    for block in unparsed_song.into_inner() {
        match block.as_rule() {
            Rule::directive => {
                directives.push(parse_directive(block));
            }
            Rule::block => {
                blocks.push(parse_block(block));
            }
            _ => { }
        }
    }
    Song {
        directives,
        blocks
    }
}

fn parse_directive(unparsed_directive: pest::iterators::Pair<Rule>) -> Directive {
    let mut directive_name = "";
    let mut deirective_value = "";
    for directive_elements in unparsed_directive.into_inner() {
        match directive_elements.as_rule() {
            Rule::name => {
                directive_name = directive_elements.as_str();
            }
            Rule::value => {
                deirective_value = directive_elements.as_str();
            }
            _ => {}
        }
    }
    Directive {
        name: directive_name.to_string(),
        value: deirective_value.to_string(),
    }
}

fn parse_blocks(unparsed_blocks: pest::iterators::Pair<Rule>) -> Vec<Block> {
    let mut blocks = Vec::new();
    for block in unparsed_blocks.into_inner() {
        let parsed_block = parse_block(block);
        blocks.push(parsed_block);
    }
    blocks
}

fn parse_block(unparsed_block: pest::iterators::Pair<Rule>) -> Block {
    let mut section_name = "";
    let mut lines:Vec<LyricLine> = Vec::new();
    for block_element in unparsed_block.into_inner() {
        match block_element.as_rule() {
            Rule::section_header => {
                section_name = block_element.as_str();
            }
            Rule::lyric_line => {
                lines.push(parse_line(block_element))
            }
            _ => { }
        }
    }
    Block {
        section_name: section_name.to_string(),
        lines
    }
}

fn parse_line(unparsed_line: pest::iterators::Pair<Rule>) -> LyricLine {
    let mut segments:Vec<Segment> = Vec::new();
    for line in unparsed_line.into_inner() {
        match line.as_rule() {
            Rule::measure => {
                let parsed_measure = parse_measure(line);
                segments.push(parsed_measure)
            }
            Rule::lyric_block => {
                let parsed_lyric_block = parse_lyric_block(line);
                segments.push(parsed_lyric_block)
            }
            _ => { }
        }
    }
    LyricLine{ segments }
}

fn parse_measure(unparsed_measure: pest::iterators::Pair<Rule>) -> Segment {
    let unparsed_chords_or_text = unparsed_measure.into_inner().next().unwrap();
    let parsed_chords_or_text = parse_chords_or_text(unparsed_chords_or_text);

    Measure(parsed_chords_or_text)
}

fn parse_lyric_block(unparsed_lyric_block: pest::iterators::Pair<Rule>) -> Segment {
    let unparsed_chords_or_text = unparsed_lyric_block.into_inner().next().unwrap();
    let parsed_chords_or_text = parse_chords_or_text(unparsed_chords_or_text);
    Inline(parsed_chords_or_text)
}

fn parse_chords_or_text(unparsed_chords_or_text: pest::iterators::Pair<Rule>) -> Vec<ChordOrText> {
    let mut chords_or_text:Vec<ChordOrText> = Vec::new();
    for unparsed_chord_or_text in unparsed_chords_or_text.into_inner() {
        match unparsed_chord_or_text.as_rule() {
            Rule::chord_token => {
                let parsed_chord_token = parse_chord_token(unparsed_chord_or_text);
                chords_or_text.push(ChordOrText::Chord(parsed_chord_token))
            }
            Rule::text_token => {
                let parsed_text_token = parse_text_token(unparsed_chord_or_text);
                chords_or_text.push(parsed_text_token)
            }
            _ => { }
        }
    }
    chords_or_text
}

fn parse_text_token(unparsed_text_token: pest::iterators::Pair<Rule>) -> ChordOrText {
    let text = unparsed_text_token.as_str().to_string();
    ChordOrText::Text(text)
}

fn parse_chord_token(unparsed_chord: pest::iterators::Pair<Rule>) -> Chord {
    let chord_elements = unparsed_chord.into_inner().next().unwrap();;
    let parsed_chord = parse_chord(chord_elements);
    parsed_chord
}

fn parse_chord(unparsed_chord_elements: pest::iterators::Pair<Rule>) -> Chord {
    let mut chord = Chord {
        root: Note {
            letter: NoteLetter::A,
            accidental: Accidental::None,
        },
        quality: None,
        extensions: Vec::new(),
        bass: None,
    };
    chord
}

#[test]
fn test_parses_simple_line(){
    let input = "[C]Hello, [G]world!";
    let parsed = LeadSheetMLParser::parse(Rule::lyric_line, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_measure(){
    let input = "| [C]Hello, [G]world! |";
    let parsed = LeadSheetMLParser::parse(Rule::measure, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_chord(){
    let input = "[C]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_complex_chords(){
    let input = "[C/G]";
    let input2 = "[C#maj7b5]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    let parsed2 = LeadSheetMLParser::parse(Rule::chord_token, input2);
    assert!(parsed.is_ok());
    assert!(parsed2.is_ok());
}

#[test]
fn test_parses_simple_text(){
    let input = "Hello, world!";
    let parsed = LeadSheetMLParser::parse(Rule::text_token, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_directive(){
    let input = "@title: Hello, world!";
    let parsed = LeadSheetMLParser::parse(Rule::directive, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_multiple_directives(){
    let input = "@title: For Absent Friends\n@artist: Genesis\n@key A Minor\n@time 4/4\n@tempo Andante";
    let parsed= LeadSheetMLParser::parse(Rule::directive_list, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_section_header(){
    let input = "#Title";
    let parsed = LeadSheetMLParser::parse(Rule::section_header, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_block(){
    let input = "| Hello, world! |";
    let parsed = LeadSheetMLParser::parse(Rule::lyric_block, input);
}

#[test]
fn test_parses_complex_block(){
    let input = "#Verse\n[D] Sunday at [D/C#] six when they [D/C] close both the gates\n[D] A [Em] wi [D] dowed [Em]pair\n[D]Still [Em]sit[D]ting [A7]there,\n[G]Wonder [Em]if they're [A]late for [D]church\nAnd its [D/C#]cold, so they [D/C]fasten their coats\n[D]And [Em]cross [D]the [Em]grass, [D]theyre [Em]al[D]ways [A7]last.";
    let parsed = LeadSheetMLParser::parse(Rule::block, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_simple_song(){
    let input = "@title: Hello, Word\n#Intro\n| [C]Hello, world! |";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_song(){
    let input = "@title: Twinkle Twinkle Little Star\n@key: C Major\n#Verse\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Chorus\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Bridge\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Outro\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_for_absent_friends(){
    let input = fs::read_to_string("SongBook/Genesis/for_absent_friends.impl").unwrap();
    let parsed = LeadSheetMLParser::parse(Rule::song, &input.as_str().trim());
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}