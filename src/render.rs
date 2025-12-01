use crate::ast::*;
use markup_engine::engine::MarkupEngine;
use std::fmt::Write;

pub trait LeadSheetRenderer {
    fn render_song(&self, engine:&dyn MarkupEngine,song: &Song) -> String;

}

pub struct DefaultLeadSheetRenderer;

impl LeadSheetRenderer for DefaultLeadSheetRenderer {
    fn render_song(&self, engine: &dyn MarkupEngine, song: &Song) -> String {
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
            let header_text = block.section_name.trim_start_matches('#');
            output.push_str(&engine.header(3, header_text));
            output.push_str(&engine.linebreak());

            let mut pre_block_text = String::new();

            for line in &block.lines {
                let pairs = render_chord_lyric_lines(line);

                for (chord_line, lyric_line) in pairs {
                    if chord_line.trim().is_empty() && lyric_line.trim().is_empty() {
                        continue;
                    }

                    pre_block_text.push_str(&chord_line);
                    pre_block_text.push('\n');
                    pre_block_text.push_str(&lyric_line);
                    pre_block_text.push('\n');
                }
            }

            if !pre_block_text.trim().is_empty() {
                let pre_block_text = pre_block_text.trim_end_matches('\n');
                output.push_str(&engine.pre_block(pre_block_text));
                output.push_str(&engine.linebreak());
            }
        }

        output
    }
}



fn render_chord_lyric_lines(line: &LyricLine) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();

    let mut chord_line = String::new();
    let mut lyric_line = String::new();
    let mut has_lyric = false; // <- new

    for segment in &line.segments {
        match segment {
            Segment::Measure(items) | Segment::Inline(items) => {
                for item in items {
                    match item {
                        ChordOrText::Chord(c) => {
                            let chord = format_chord(c.clone());
                            let width = chord.len().max(4); // same as before

                            write!(&mut chord_line, "{:width$}", chord, width = width).unwrap();

                            // 👇 only pad lyric line *after* lyrics have started
                            if has_lyric {
                                lyric_line.push_str(&" ".repeat(width));
                            }
                        }

                        ChordOrText::Text(text) => {
                            // Handle embedded newlines like before
                            let parts: Vec<&str> = text.split('\n').collect();

                            for (i, part) in parts.iter().enumerate() {
                                if i > 0 {
                                    // flush current visual line
                                    result.push((chord_line.clone(), lyric_line.clone()));
                                    chord_line.clear();
                                    lyric_line.clear();
                                    has_lyric = false; // reset for the new line
                                }

                                // respect any spaces the user put in `part` explicitly
                                let len = part.len();
                                chord_line.push_str(&" ".repeat(len));
                                lyric_line.push_str(part);
                                if !part.is_empty() {
                                    has_lyric = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !chord_line.is_empty() || !lyric_line.is_empty() {
        result.push((chord_line, lyric_line));
    }

    result
}





fn format_chord(chord: Chord) -> String {
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
