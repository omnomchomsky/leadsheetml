use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;
use crate::ast::*;

#[derive(Parser)]
#[grammar = "leadsheetml.pest"]
pub struct LeadSheetMLParser;

pub fn parse_song_from_str(input: &str) -> Song {
    let unparsed_song = LeadSheetMLParser::parse(Rule::song, input).unwrap().next();
    let parsed_song = parse_song(unparsed_song.unwrap());
    parsed_song
}

pub fn parse_song(unparsed_song: pest::iterators::Pair<Rule>) -> Song {
    let mut directives:HashMap<String, String> =  HashMap::new();
    let mut blocks:Vec<Block> = Vec::new();
    for song_elements in unparsed_song.into_inner() {
        match song_elements.as_rule() {
            Rule::directive_list => {
                for unparsed_directive in song_elements.into_inner() {
                    let directive = parse_directive(unparsed_directive);
                    directives.insert(directive.name, directive.value);
                }
            }
            Rule::blocks => {
                blocks = parse_blocks(song_elements);
            }
            _ => { }
        }
    }
    Song {
        directives,
        blocks
    }
}

pub fn parse_directive(unparsed_directive: pest::iterators::Pair<Rule>) -> Directive {
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

pub fn parse_blocks(unparsed_blocks: pest::iterators::Pair<Rule>) -> Vec<Block> {
    let mut blocks = Vec::new();
    for block in unparsed_blocks.into_inner() {
        let parsed_block = parse_block(block);
        blocks.push(parsed_block);
    }
    blocks
}

pub fn parse_block(unparsed_block: pest::iterators::Pair<Rule>) -> Block {
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
            _ => { panic!("Invalid block element: {:?}", block_element.as_rule())}
        }
    }
    Block {
        section_name: section_name.to_string(),
        lines
    }
}

pub fn parse_line(unparsed_line: pest::iterators::Pair<Rule>) -> LyricLine {
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
            _ => { panic!("Invalid line segment: {:?}", line.as_rule()) }
        }
    }
    LyricLine{ segments }
}

pub fn parse_measure(unparsed_measure: pest::iterators::Pair<Rule>) -> Segment {
    let chords_or_text = parse_line_lyric(unparsed_measure);
    Segment::Measure(chords_or_text)
}

pub fn parse_lyric_block(unparsed_lyric_block: pest::iterators::Pair<Rule>) -> Segment {
    let chords_or_text = parse_line_lyric(unparsed_lyric_block);
    Segment::Inline(chords_or_text)
}
pub fn parse_line_lyric(unparsed_measure: pest::iterators::Pair<Rule>) -> Vec<ChordOrText> {
    let mut chords_or_text:Vec<ChordOrText> = Vec::new();
    for measure_element in unparsed_measure.into_inner() {
        match measure_element.as_rule() {
            Rule::chord_or_text => {
                let chord_or_text = parse_chords_or_text(measure_element);
                chords_or_text.push(chord_or_text);
            }
            _ => { panic!("Invalid measure element: {:?}", measure_element.as_rule())}
        }
    }
    chords_or_text
}


pub fn parse_chords_or_text(unparsed_chords_or_text: pest::iterators::Pair<Rule>) -> ChordOrText {
    let mut chord_or_text:ChordOrText = ChordOrText::Text("".to_string());
    for unparsed_chord_or_text in unparsed_chords_or_text.into_inner() {
        match unparsed_chord_or_text.as_rule() {
            Rule::chord_token => {
                let parsed_chord = parse_chord_token(unparsed_chord_or_text);
                chord_or_text = ChordOrText::Chord(parsed_chord)
            }
            Rule::text_token => {
                let parsed_text_token = parse_text_token(unparsed_chord_or_text);
                chord_or_text = parsed_text_token
            }
            _ => { panic!("Invalid chord or text: {:?}", unparsed_chord_or_text.as_rule())}
        }
    }
    chord_or_text
}

pub fn parse_text_token(unparsed_text_token: pest::iterators::Pair<Rule>) -> ChordOrText {
    let text = unparsed_text_token.as_str().to_string();
    ChordOrText::Text(text)
}

pub fn parse_chord_token(unparsed_chord: pest::iterators::Pair<Rule>) -> Chord {
    let chord_elements = unparsed_chord.into_inner().next().unwrap();
    let parsed_chord = parse_chord(chord_elements);
    parsed_chord
}

pub fn parse_chord(unparsed_chord: pest::iterators::Pair<Rule>) -> Chord {
    let mut chord = Chord {
        root: Note {
            letter: NoteLetter::A,
            accidental: Accidental::None,
        },
        quality: None,
        extensions: Vec::new(),
        bass: None
    };
    for chord_element in unparsed_chord.into_inner() {
        match chord_element.as_rule() {
            Rule::chord_elements=> {
                chord = parse_chord_element(chord_element)
            }
            Rule::slash_chord => {
                let slash_chord_note = parse_slash_chord(chord_element);
                chord.bass = slash_chord_note;
            }
            _ => { panic!("Invalid chord element: {:?}", chord_element.as_rule())}
        }
    }
    chord
}

pub fn parse_slash_chord(unparsed_slash_chord_note: pest::iterators::Pair<Rule>) -> Option<Note> {
    let slash_chord_elements = unparsed_slash_chord_note.into_inner().skip(1).next().unwrap();
    let parsed_slash_chord_note = parse_note(slash_chord_elements);
    Some(parsed_slash_chord_note)
}


pub fn parse_chord_element(unparsed_chord_elements: pest::iterators::Pair<Rule>) -> Chord {
    let mut root =  Note {
        letter: NoteLetter::A,
        accidental: Accidental::None,
    };
    let mut quality:Option<String> = None;
    let mut extensions:Vec<Option<String>> = Vec::new();

    for chord_element in unparsed_chord_elements.into_inner() {
        match chord_element.as_rule() {
            Rule::key => {
                root = parse_note(chord_element)
            }
            Rule::quality => {
                quality = parse_quality(chord_element);
            }
            Rule::extension => {
                extensions.push(parse_extension(chord_element))
            }
            _ => { panic!("Invalid chord element: {:?}", chord_element.as_rule()) }
        }
    }
    Chord {
        root,
        quality,
        extensions,
        bass: None
    }
}

pub fn parse_note(unparsed_note: pest::iterators::Pair<Rule>) -> Note {
    let mut note = Note {
        letter: NoteLetter::A,
        accidental: Accidental::None
    };
    for note_element in unparsed_note.into_inner(){
        match note_element.as_rule() {
            Rule::note => {
                note.letter = parse_letter(note_element)
            }
            Rule::accidental => {
                note.accidental = parse_accidental(note_element)
            }
            _ => { panic!("Invalid note element: {:?}", note_element.as_rule())}
        }
    }
    note
}

pub fn parse_letter(unparsed_letter: pest::iterators::Pair<Rule>) -> NoteLetter {
    let letter = unparsed_letter.as_str();
    match letter {
        "A" => NoteLetter::A,
        "a" => NoteLetter::A,
        "B" => NoteLetter::B,
        "b" => NoteLetter::B,
        "C" => NoteLetter::C,
        "c" => NoteLetter::C,
        "D" => NoteLetter::D,
        "d" => NoteLetter::D,
        "E" => NoteLetter::E,
        "e" => NoteLetter::E,
        "F" => NoteLetter::F,
        "f" => NoteLetter::F,
        "G" => NoteLetter::G,
        "g" => NoteLetter::G,
        _ => panic!("Invalid note letter: {}", letter)
    }
}

pub fn parse_accidental(unparsed_accidental: pest::iterators::Pair<Rule>) -> Accidental {
    let accidental = unparsed_accidental.as_str();
    match accidental {
        "#" => Accidental::Sharp,
        "b" => Accidental::Flat,
        _ => Accidental::None
    }
}

pub fn parse_quality(unparsed_quality: pest::iterators::Pair<Rule>) -> Option<String> {
    let quality = unparsed_quality.as_str();
    match quality {
        "maj" => Some("maj".to_string()),
        "min" => Some("min".to_string()),
        "dim" => Some("dim".to_string()),
        "aug" => Some("aug".to_string()),
        "m" => Some("m".to_string()),
        "+" => Some("+".to_string()),
        _ => None
    }
}
pub fn parse_extension(unparsed_extension: pest::iterators::Pair<Rule>) -> Option<String> {
    let extension = unparsed_extension.as_str();
    match extension {
        "7" => Some("7".to_string()),
        "9" => Some("9".to_string()),
        "maj7" => Some("maj7".to_string()),
        "maj9" => Some("maj9".to_string()),
        "min7" => Some("min7".to_string()),
        "min9" => Some("min9".to_string()),
        "11" => Some("11".to_string()),
        "13" => Some("13".to_string()),
        "b5" => Some("b5".to_string()),
        "b9" => Some("b9".to_string()),
        "b11" => Some("b11".to_string()),
        "b13" => Some("b13".to_string()),
        "#5" => Some("#5".to_string()),
        "#9" => Some("#9".to_string()),
        "#11" => Some("#11".to_string()),
        "#13" => Some("#13".to_string()),
        "dim7" => Some("dim7".to_string()),
        "dim9" => Some("dim9".to_string()),
        "sus2" => Some("sus2".to_string()),
        "sus4" => Some("sus4".to_string()),
        "dim" => Some("dim".to_string()),
        "aug" => Some("aug".to_string()),
        _ => None
    }
}