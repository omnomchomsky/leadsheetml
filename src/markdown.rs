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

pub fn render_song(song: &Song) -> String {
    let mut output = String::new();
    let declarative = &song.directives;
    let blocks = &song.blocks; // Assuming typo fixed to song.blocks

    // Render title and artist
    let title = declarative.get("title").unwrap();
    output.push_str(&wrap_markdown("# ".to_string(), title.to_string(), "".to_string(), true));
    if let Some(artist) = declarative.get("artist") {
        output.push_str(&wrap_markdown("* ".to_string(), artist.to_string(), " *".to_string(), true));
    }

    // Render each block
    for block in blocks {
        // Strip leading # from section name
        let section_name = block.section_name.trim_start_matches('#');
        output.push_str(&wrap_markdown("## ".to_string(), section_name.to_string(), "".to_string(), true));

        // Accumulate chords and lyrics for each line
        for line in &block.lines {
            let mut chord_line = String::new();
            let mut lyric_line = String::new();

            for segment in &line.segments {
                match segment {
                    Segment::Measure(chords_or_text) | Segment::Inline(chords_or_text) => {
                        for ct in chords_or_text {
                            match ct {
                                ChordOrText::Chord(chord) => {
                                    let chord_string = format_chord(chord.clone());
                                    // Use a fixed width for better alignment (e.g., 8 chars)
                                    chord_line.push_str(&format!("{:<8}", chord_string));
                                    lyric_line.push_str(&" ".repeat(chord_string.len().min(8)));
                                }
                                ChordOrText::Text(text) => {
                                    chord_line.push_str(&" ".repeat(text.len()));
                                    lyric_line.push_str(text);
                                }
                            }
                        }
                    }
                }
            }
            // Add the completed line to output
            if !chord_line.trim().is_empty() {
                output.push_str(&chord_line);
                output.push('\n');
            }
            if !lyric_line.trim().is_empty() {
                output.push_str(&lyric_line);
                output.push('\n');
            }
            output.push('\n'); // Extra newline between lines
        }
    }
    output
}
