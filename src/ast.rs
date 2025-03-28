use std::collections::HashMap;

#[derive(Debug)]
pub struct Song {
    pub directives: HashMap<String, String>,
    pub blocks: Vec<Block>,
}
#[derive(Debug)]
pub struct Directive {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Block {
    pub section_name: String,
    pub lines: Vec<LyricLine>,
}

#[derive(Debug)]
pub struct LyricLine {
    pub segments: Vec<Segment>,
}

#[derive(Debug)]
pub enum Segment {
    Measure(Vec<ChordOrText>),
    Inline(Vec<ChordOrText>),
}

#[derive(Debug)]
pub enum ChordOrText {
    Chord(Chord),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub root: Note,
    pub inversion: Option<String>,         //6, 6/9
    pub quality: Option<String>,           // "maj", "min", "dim", etc.
    pub extensions: Vec<Option<String>>,   // "7", "9", "b5", etc.
    pub bass: Option<Note>,                // For slash chords
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub letter: NoteLetter,          // A-G
    pub accidental: Accidental,      // Sharp, Flat, Natural
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteLetter {
    A, B, C, D, E, F, G
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Accidental {
    Sharp,
    Flat,
    None,
}