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
        output.push_str(&wrap_markdown("*".to_string(), artist.to_string(), "*".to_string(), true));
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
                                    chord_line.push_str(&format!("{:2}", chord_string));
                                    lyric_line.push_str(&" ".repeat(chord_string.len()));
                                }
                                ChordOrText::Text(text) => {

                                    chord_line.push_str(&" ".repeat(text.len()));
                                    let arr:Vec<String> = simple_split(text, '\n');
                                    lyric_line.push_str(arr[0].as_str());

                                    if contains_newline(&text) {
                                        output.push_str(&wrap_markdown("".to_string(), chord_line.to_string(), "".to_string(), true));
                                        output.push_str(&wrap_markdown("".to_string(), lyric_line.to_string(), "".to_string(), true));
                                        chord_line = String::new();
                                        lyric_line = String::new();
                                    }
                                    if arr[1].len() > 0 {
                                        lyric_line.push_str(&wrap_markdown(" ".to_string(), arr[1].to_string(), " ".to_string(), false));
                                        chord_line.push_str(&wrap_markdown("   ".to_string(), " ".repeat(arr[1].len()), " ".to_string(), false));
                                    }
                                }
                            }
                        }
                    }
                }
                if !chord_line.trim().is_empty() || !lyric_line.trim().is_empty() {
                    output.push_str(&wrap_markdown(" ".to_string(), chord_line.to_string(), " ".to_string(), true));
                    output.push_str(&wrap_markdown("".to_string(), lyric_line.to_string(), "".to_string(), true));
                }
            }
        }
    }
    output
}

fn contains_newline(p0: &String) -> bool {
    p0.contains('\n')
}

fn simple_split(p0: &String, split_char:char) -> Vec<String> {
    let mut arr:Vec<String> = Vec::new();
    if p0.contains(split_char) {
        let some_arr = p0.split(split_char);
        for s in some_arr {
            arr.push(s.to_string());
        }
    }
    else {
        arr.push(p0.to_string());
        arr.push("".to_string());
    }
    arr
}
