use crate::ast::*;

fn wrap_markdown(front:String, string:String, mut back:String, newline:bool) -> String {
    if newline {
        back.push('\n');
    }
    (front + &*string + &*back).to_string()
}

fn format_chord(chord: Chord) -> String {
    let mut s = format!("{:?}", chord.root.letter);

    match chord.root.accidental {
        Accidental::Sharp => s.push('#'),
        Accidental::Flat => s.push('b'),
        Accidental::Natural => s.push_str("♮"),
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
            Accidental::Natural => s.push_str("♮"),
            Accidental::None => {}
        }
    }
    s
}

pub fn render_song(song: &Song) -> String {
    let mut output = String::new();
    let declarative = &song.directives;
    let blocks = &song.blocks;
    let title = declarative.get("title").unwrap();
    output.push_str(wrap_markdown("# ".to_string(), title.to_string(), "".to_string(), true).as_str());
    let artist = declarative.get("artist").unwrap();
    output.push_str(wrap_markdown("* ".to_string(), artist.to_string(), " *".to_string(), true).as_str());
    for block in blocks {
        output.push_str(wrap_markdown("## ".to_string(), block.section_name.clone(), "".to_string(), true).as_str());
        let mut chord_line = String::new();
        let mut lyric_line = String::new();

        for line in block.lines.iter() {
            for segment in line.segments.iter() {
                match segment {
                    Segment::Measure(chords_or_text) | Segment::Inline(chords_or_text) => {
                        for ct in chords_or_text {
                            match ct {
                                ChordOrText::Chord(chord) => {
                                    let chord_string = format_chord(chord.clone());
                                    chord_line.push_str(&format!("{:<width$}", chord_string, width=chord_string.len()));
                                    lyric_line.push_str(&" ".repeat(chord_string.len()));
                                }
                                ChordOrText::Text(text) => {
                                    chord_line.push_str(&" ".repeat(text.len()));
                                    lyric_line.push_str(&text);
                                }
                            }

                            chord_line.push_str("  ");
                            lyric_line.push_str("  ");
                        }
                    }
                }
                output.push_str(wrap_markdown("| ".to_string(), chord_line.clone(), "| ".to_string(), false).as_str());
                output.push_str(wrap_markdown("| ".to_string(), lyric_line.clone(), "| ".to_string(), false).as_str());
                chord_line = String::new();
                lyric_line = String::new();
            }
        }
    }
    output
}
