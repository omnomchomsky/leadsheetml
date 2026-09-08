use crate::ast::*;
use markup_engine::engine::MarkupEngine;
use markup_engine::{HtmlEngine, MarkdownEngine};

/// Destination formatting runs only after the plain-text rows are laid out.
pub trait LeadSheetFormat: MarkupEngine {
    fn format_rows(&self, rows: &[RenderedLine]) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub struct RenderedLine {
    pub chords: String,
    pub lyrics: String,
}

impl LeadSheetFormat for HtmlEngine {
    fn format_rows(&self, rows: &[RenderedLine]) -> String {
        let text = rows
            .iter()
            .map(|row| {
                format!(
                    "{}\n{}",
                    self.bold(&row.chords),
                    self.line_segment(&row.lyrics)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        self.pre_block(&text)
    }
}

impl LeadSheetFormat for MarkdownEngine {
    fn format_rows(&self, rows: &[RenderedLine]) -> String {
        let text = rows
            .iter()
            .map(|row| format!("{}\n{}", row.chords, row.lyrics))
            .collect::<Vec<_>>()
            .join("\n");
        // A longer fence keeps literal backticks in lyrics inside the code block.
        let longest_run = text.split(|c| c != '`').map(str::len).max().unwrap_or(0);
        let fence = "`".repeat(3.max(longest_run + 1));
        format!("{fence}\n{text}\n{fence}")
    }
}

pub trait LeadSheetRenderer {
    fn render_song(&self, engine: &dyn LeadSheetFormat, song: &Song) -> String;
}

pub struct DefaultLeadSheetRenderer;

impl LeadSheetRenderer for DefaultLeadSheetRenderer {
    fn render_song(&self, engine: &dyn LeadSheetFormat, song: &Song) -> String {
        let mut output = String::new();
        if let Some(title) = song.directives.get("title") {
            output.push_str(&engine.header(1, title));
            output.push_str(&engine.linebreak());
        }
        if let Some(artist) = song.directives.get("artist") {
            output.push_str(&engine.italic(artist));
            output.push_str(&engine.linebreak());
        }
        for block in &song.blocks {
            output.push_str(&engine.header(3, block.section_name.trim_start_matches('#')));
            output.push_str(&engine.linebreak());
            let rows: Vec<_> = block
                .lines
                .iter()
                .flat_map(extract_lines)
                .filter(|line| !line.chords.is_empty() || !line.lyrics.trim().is_empty())
                .map(layout_line)
                .collect();
            if !rows.is_empty() {
                output.push_str(&engine.format_rows(&rows));
                output.push_str(&engine.linebreak());
            }
        }
        output
    }
}

#[derive(Debug, Default)]
struct AnchoredLine {
    lyrics: String,
    chords: Vec<ChordAnchor>,
}

#[derive(Debug)]
struct ChordAnchor {
    // Character position in the original lyric text, before collision padding.
    position: usize,
    label: String,
}

fn extract_lines(line: &LyricLine) -> Vec<AnchoredLine> {
    let mut lines = Vec::new();
    let mut current = AnchoredLine::default();
    let mut position = 0;
    for segment in &line.segments {
        let (Segment::Measure(items) | Segment::Inline(items)) = segment;
        for item in items {
            match item {
                ChordOrText::Chord(chord) => current.chords.push(ChordAnchor {
                    position,
                    label: format_chord(chord),
                }),
                ChordOrText::Text(text) => {
                    for c in text.replace("\r\n", "\n").chars() {
                        match c {
                            '\n' => {
                                lines.push(std::mem::take(&mut current));
                                position = 0;
                            }
                            '\t' => {
                                let spaces = 4 - position % 4;
                                current.lyrics.push_str(&" ".repeat(spaces));
                                position += spaces;
                            }
                            _ => {
                                current.lyrics.push(c);
                                position += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    lines.push(current);
    lines
}

fn layout_line(line: AnchoredLine) -> RenderedLine {
    let mut lyrics: Vec<char> = line.lyrics.chars().collect();
    let mut chords = String::new();
    let mut chord_end = 0;
    let mut padding = 0;
    for anchor in line.chords {
        let mut position = anchor.position + padding;
        // Keep one space between labels; move this lyric anchor and all later
        // anchors together when the previous label occupies its column.
        let minimum = if chords.is_empty() { 0 } else { chord_end + 1 };
        if position < minimum {
            let extra = minimum - position;
            lyrics.splice(position..position, std::iter::repeat_n(' ', extra));
            padding += extra;
            position += extra;
        }
        chords.push_str(&" ".repeat(position - chord_end));
        chords.push_str(&anchor.label);
        chord_end = position + anchor.label.chars().count();
    }
    RenderedLine {
        chords,
        lyrics: lyrics.into_iter().collect(),
    }
}

fn format_chord(chord: &Chord) -> String {
    let mut s = format!("{:?}", chord.root.letter);

    match chord.root.accidental {
        Accidental::Sharp => s.push('#'),
        Accidental::Flat => s.push('b'),
        Accidental::None => {}
    }

    if let Some(q) = &chord.quality {
        s.push_str(q);
    }

    for ext in &chord.extensions {
        if let Some(e) = ext {
            s.push_str(e);
        }
    }

    if let Some(bass) = &chord.bass {
        s.push('/');
        s.push_str(&format!("{:?}", bass.letter));
        match bass.accidental {
            Accidental::Sharp => s.push('#'),
            Accidental::Flat => s.push('b'),
            Accidental::None => {}
        }
    }
    s
}
