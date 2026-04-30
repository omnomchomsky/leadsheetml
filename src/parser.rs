use std::collections::HashMap;
use pest::{Parser};
use pest_derive::Parser;
use crate::ast::*;

#[derive(Parser)]
#[grammar = "leadsheetml.pest"]
pub struct LeadSheetMLParser;

#[derive(Debug)]
pub enum LeadSheetMLError{
    Pest(Box<pest::error::Error<Rule>>),
    Syntax {
        message: String,
        rule: Option<Rule>,
        span: Option<(usize, usize)>
    },
    Internal{
        message: String
    },
}

pub type ParseResult<T> = Result<T, LeadSheetMLError>;

pub fn parse_song_from_str(input: &str) -> ParseResult<Song> {
    let mut pairs = LeadSheetMLParser::parse(Rule::song, input)
        .map_err(|e| LeadSheetMLError::Pest(Box::new(e)))?;
    let song = pairs.next().ok_or_else(|| LeadSheetMLError::Internal {
        message: "expected top-level song rule".to_string()
    })?;
    parse_song(song)
}

pub fn parse_song(unparsed_song: pest::iterators::Pair<Rule>) -> ParseResult<Song> {
    let mut directives:HashMap<String, String> =  HashMap::new();
    let mut blocks:Vec<Block> = Vec::new();
    for song_elements in unparsed_song.into_inner() {
        match song_elements.as_rule() {
            Rule::directive_list => {
                for unparsed_directive in song_elements.into_inner() {
                    let directive = parse_directive(unparsed_directive)?;
                    directives.insert(directive.name, directive.value);
                }
            }
            Rule::blocks => {
                blocks = parse_blocks(song_elements)?;
            }
            _ => { }
        }
    }
    Ok(Song {
        directives,
        blocks
    })
}

pub fn parse_directive(unparsed_directive: pest::iterators::Pair<Rule>) -> ParseResult<Directive> {
    let mut directive_name = "";
    let mut directive_value = "";
    for directive_elements in unparsed_directive.into_inner() {
        match directive_elements.as_rule() {
            Rule::name => {
                directive_name = directive_elements.as_str();
            }
            Rule::value => {
                directive_value = directive_elements.as_str();
            }
            _ => {}
        }
    }
    Ok(Directive {
        name: directive_name.to_string(),
        value: directive_value.to_string(),
    })
}

pub fn parse_blocks(unparsed_blocks: pest::iterators::Pair<Rule>) -> ParseResult<Vec<Block>> {
    let mut blocks = Vec::new();
    for block in unparsed_blocks.into_inner() {
        blocks.push(parse_block(block)?);
    }
    Ok(blocks)
}

pub fn parse_block(unparsed_block: pest::iterators::Pair<Rule>) -> ParseResult<Block> {
    let mut section_name = "";
    let mut lines:Vec<LyricLine> = Vec::new();
    for block_element in unparsed_block.into_inner() {
        match block_element.as_rule() {
            Rule::section_header => {
                section_name = block_element.as_str();
            }
            Rule::lyric_line => {
                lines.push(parse_line(block_element)?)
            }
            _ => return internal(format!("Invalid block elemement: {:?}", block_element.as_rule()))
        }
    }
    Ok(Block {
        section_name: section_name.to_string(),
        lines
    })
}

pub fn parse_line(unparsed_line: pest::iterators::Pair<Rule>) -> ParseResult<LyricLine> {
    let mut segments:Vec<Segment> = Vec::new();
    for line in unparsed_line.into_inner() {
        match line.as_rule() {
            Rule::measure => {
                segments.push(parse_measure(line)?)
            }
            Rule::lyric_block => {
                segments.push(parse_lyric_block(line)?)
            }
            _ => { return Err(LeadSheetMLError::Internal {
                message:format!("Invalid line: {:?}", line.as_rule())
            }) }
        }
    }
    Ok(LyricLine{ segments })
}

pub fn parse_measure(unparsed_measure: pest::iterators::Pair<Rule>) -> ParseResult<Segment> {
    let chords_or_text = parse_line_lyric(unparsed_measure)?;
    Ok(Segment::Measure(chords_or_text))
}

pub fn parse_lyric_block(unparsed_lyric_block: pest::iterators::Pair<Rule>) -> ParseResult<Segment> {
    let chords_or_text = parse_line_lyric(unparsed_lyric_block)?;
    Ok(Segment::Inline(chords_or_text))
}

pub fn parse_line_lyric(unparsed_measure: pest::iterators::Pair<Rule>) -> ParseResult<Vec<ChordOrText>> {
    let mut chords_or_text:Vec<ChordOrText> = Vec::new();
    for measure_element in unparsed_measure.into_inner() {
        match measure_element.as_rule() {
            Rule::chord_or_text => {
                let chord_or_text = parse_chords_or_text(measure_element);
                chords_or_text.push(chord_or_text?);
            }
            _ => return internal("Invalid lyric line".to_string())
        }
    }
    Ok(chords_or_text)
}


pub fn parse_chords_or_text(pair: pest::iterators::Pair<Rule>) -> ParseResult<ChordOrText> {
    let mut inner = pair.into_inner();

    let first = inner.next().ok_or_else(|| LeadSheetMLError::Internal {
        message: "Chord or text token has no inner elements".to_string()
    })?;

    match first.as_rule(){
        Rule::chord_token => Ok(ChordOrText::Chord(parse_chord_token(first)?)),
        Rule::text_token => parse_text_token(first),
        _ => internal(format!("Invalid chord or text token: {:?}", first.as_rule())),
    }
}

pub fn parse_text_token(unparsed_text_token: pest::iterators::Pair<Rule>) -> ParseResult<ChordOrText> {
    let text = unparsed_text_token.as_str().to_string();
    Ok(ChordOrText::Text(text))
}

pub fn parse_chord_token(unparsed_chord: pest::iterators::Pair<Rule>) -> ParseResult<Chord> {
    let chord = unparsed_chord
        .into_inner()
        .next()
        .ok_or_else(|| LeadSheetMLError::Internal{
            message: "Chord token has no inner elements".to_string()
        })?;
    let parsed_chord = parse_chord(chord)?;
    Ok(parsed_chord)
}

pub fn parse_chord(unparsed_chord: pest::iterators::Pair<Rule>) -> ParseResult<Chord> {
    let mut chord = Chord {
        root: Note {
            letter: NoteLetter::A,
            accidental: Accidental::None,
        },
        inversion: None,
        quality: None,
        extensions: Vec::new(),
        bass: None
    };
    for chord_element in unparsed_chord.into_inner() {
        match chord_element.as_rule() {
            Rule::chord_elements=> {
                chord = parse_chord_element(chord_element)?;
            }
            Rule::slash_chord => {
                let slash_chord_note = parse_slash_chord(chord_element)?;
                chord.bass = slash_chord_note;
            }
            _ => return internal(format!("Invalid chord element: {:?}", chord_element.as_rule()))
        }
    }
    Ok(chord)
}

pub fn parse_slash_chord(unparsed_slash_chord_note: pest::iterators::Pair<Rule>) -> ParseResult<Option<Note>> {
    let slash_chord = unparsed_slash_chord_note
        .into_inner()
        .skip(1) //Skip the slash
        .next()
        .ok_or_else(|| LeadSheetMLError::Internal {
            message: "Slash chord has no inner elements".to_string()
        })?;
    let parsed_slash_chord_note = parse_note(slash_chord)?;
    Ok(Some(parsed_slash_chord_note))
}


pub fn parse_chord_element(unparsed_chord_elements: pest::iterators::Pair<Rule>) -> ParseResult<Chord> {
    let mut root =  Note {
        letter: NoteLetter::A,
        accidental: Accidental::None,
    };
    let mut quality:Option<String> = None;
    let mut extensions:Vec<Option<String>> = Vec::new();
    let mut inversion:Option<String> = None;

    for chord_element in unparsed_chord_elements.into_inner() {
        match chord_element.as_rule() {
            Rule::key => {
                root = parse_note(chord_element)?
            }
            Rule::inversion => {
                inversion = parse_inversion(chord_element)?
            }
            Rule::quality => {
                quality = parse_quality(chord_element)?;
            }
            Rule::extension => {
                extensions.push(parse_extension(chord_element)?)
            }
            _ => return internal(format!("Invalid chord element: {:?}", chord_element.as_rule()))

        }
    }
    Ok(Chord {
        root,
        inversion,
        quality,
        extensions,
        bass: None
    })
}

pub fn parse_note(unparsed_note: pest::iterators::Pair<Rule>) -> ParseResult<Note> {
    let mut note = Note {
        letter: NoteLetter::A,
        accidental: Accidental::None
    };
    for note_element in unparsed_note.into_inner(){
        match note_element.as_rule() {
            Rule::note => {
                note.letter = parse_letter(note_element)?
            }
            Rule::accidental => {
                note.accidental = parse_accidental(note_element)?
            }
            _ => return internal(format!("Invalid note element: {:?}", note_element.as_rule()))
        }

    }
    Ok(note)
}

pub fn parse_letter(unparsed_letter: pest::iterators::Pair<Rule>) -> ParseResult<NoteLetter> {
    let letter = unparsed_letter.as_str();
    match letter {
        "A" => Ok(NoteLetter::A),
        "a" => Ok(NoteLetter::A),
        "B" => Ok(NoteLetter::B),
        "b" => Ok(NoteLetter::B),
        "C" => Ok(NoteLetter::C),
        "c" => Ok(NoteLetter::C),
        "D" => Ok(NoteLetter::D),
        "d" => Ok(NoteLetter::D),
        "E" => Ok(NoteLetter::E),
        "e" => Ok(NoteLetter::E),
        "F" => Ok(NoteLetter::F),
        "f" => Ok(NoteLetter::F),
        "G" => Ok(NoteLetter::G),
        "g" => Ok(NoteLetter::G),
        _ => internal(format!("Invalid NoteLetter: {}", letter))
    }
}

pub fn parse_accidental(unparsed_accidental: pest::iterators::Pair<Rule>) -> ParseResult<Accidental> {
    let accidental = unparsed_accidental.as_str();
    match accidental {
        "#" => Ok(Accidental::Sharp),
        "b" => Ok(Accidental::Flat),
        _ => Ok(Accidental::None)
    }
}

pub fn parse_inversion(unparsed_inversion: pest::iterators::Pair<Rule>) -> ParseResult<Option<String>> {
    let inversion = unparsed_inversion.as_str();
    match inversion {
        "6" => Ok(Some("6".to_string())),
        "6/9" => Ok(Some("6/9".to_string())),
        _ => Ok(None)
    }
}
pub fn parse_quality(unparsed_quality: pest::iterators::Pair<Rule>) -> ParseResult<Option<String>> {
    let quality = unparsed_quality.as_str();
    match quality {
        "maj" => Ok(Some("maj".to_string())),
        "min" => Ok(Some("min".to_string())),
        "dim" => Ok(Some("dim".to_string())),
        "aug" => Ok(Some("aug".to_string())),
        "m" => Ok(Some("m".to_string())),
        "+" => Ok(Some("+".to_string())),
        _ => Ok(None)
    }
}
pub fn parse_extension(unparsed_extension: pest::iterators::Pair<Rule>) -> ParseResult<Option<String>> {
    let extension = unparsed_extension.as_str();
    match extension {
        "7" => Ok(Some("7".to_string())),
        "9" => Ok(Some("9".to_string())),
        "maj7" => Ok(Some("maj7".to_string())),
        "maj9" => Ok(Some("maj9".to_string())),
        "min7" => Ok(Some("min7".to_string())),
        "min9" => Ok(Some("min9".to_string())),
        "11" => Ok(Some("11".to_string())),
        "13" => Ok(Some("13".to_string())),
        "b5" => Ok(Some("b5".to_string())),
        "b9" => Ok(Some("b9".to_string())),
        "b11" => Ok(Some("b11".to_string())),
        "b13" => Ok(Some("b13".to_string())),
        "#5" => Ok(Some("#5".to_string())),
        "#9" => Ok(Some("#9".to_string())),
        "#11" => Ok(Some("#11".to_string())),
        "#13" => Ok(Some("#13".to_string())),
        "dim7" => Ok(Some("dim7".to_string())),
        "dim9" => Ok(Some("dim9".to_string())),
        "sus2" => Ok(Some("sus2".to_string())),
        "sus4" => Ok(Some("sus4".to_string())),
        "dim" => Ok(Some("dim".to_string())),
        "aug" => Ok(Some("aug".to_string())),
        _ => Ok(None)
    }
}

fn internal<T>(message: impl Into<String>) -> ParseResult<T> {
    Err(LeadSheetMLError::Internal {
        message: message.into()
    })
}