use crate::ast::*;

const CHROMATIC_SCALE:[&str; 12] = [
"C", "Db", "D", "Eb", "E", "F",
"Gb", "G", "Ab", "A", "Bb", "B"
];

fn relative_transpose(chord: Chord, semitones: isize) -> Chord {
    if semitones == 0 {
        chord
    } else if semitones < 0 {
        let steps = 12 - (semitones.abs() as usize % 12);
        let transposed_chord = transpose_chord(chord, steps);
        transposed_chord
    } else {
        let steps = (semitones % 12) as usize;
        let transposed_chord =transpose_chord(chord, steps);
        transposed_chord
    }
}

fn transpose_chord(chord: Chord, steps: usize) -> Chord {
    let new_root = relative_note(chord.root.clone(), steps);
        let new_bass = if chord.bass.is_none() {
            None
        } else {
            Some(relative_note(chord.bass.clone().unwrap(), steps))
        };
    let new_extensions = chord.extensions.clone();
    let new_quality = chord.quality.clone();
    Chord {
        root: new_root,
        inversion: chord.inversion,
        quality: new_quality,
        extensions: new_extensions,
        bass: new_bass,
    }
}

fn relative_note(chord_note: Note, steps: usize) -> Note {
    let original_key = note_to_string(chord_note.letter, chord_note.accidental);
    let from_index = CHROMATIC_SCALE.iter().position(|x| x == &find_enharmonic_spelling(&original_key)).expect(format!("Expected {} key to be in the chromatic scale", original_key).as_str());
    let new_index = (from_index + steps) % 12;
    let transposed_key = CHROMATIC_SCALE[new_index];
    string_to_note(transposed_key)
}
fn note_to_string(letter: NoteLetter, accidental: Accidental) -> String {

    let mut output = match letter {
        NoteLetter::A => "A".to_string(),
        NoteLetter::B => "B".to_string(),
        NoteLetter::C => "C".to_string(),
        NoteLetter::D => "D".to_string(),
        NoteLetter::E => "E".to_string(),
        NoteLetter::F => "F".to_string(),
        NoteLetter::G => "G".to_string(),
    };

    match accidental {
        Accidental::Sharp => output.push_str("#"),
        Accidental::Flat => output.push_str("b"),
        Accidental::None => output.push_str(""),
    }
    output
}

fn string_to_note(key: &str) -> Note {
    let mut note = Note{
        letter: NoteLetter::A,
        accidental: Accidental::None
    };
    match key.chars().next().unwrap() {
        'A' => note.letter = NoteLetter::A,
        'B' => note.letter = NoteLetter::B,
        'C' => note.letter = NoteLetter::C,
        'D' => note.letter = NoteLetter::D,
        'E' => note.letter = NoteLetter::E,
        'F' => note.letter = NoteLetter::F,
        'G' => note.letter = NoteLetter::G,
        _ => { }
    }

    if key.len() > 1 {
        match key.chars().nth(1).unwrap() {
            '#' => note.accidental = Accidental::Sharp,
            'b' => note.accidental = Accidental::Flat,
            _ => {}
        }
    }
    note
}

fn find_enharmonic_spelling(key: &str) -> String {
   match key {
       "Cb" => "B",
       "C#" => "Db",
       "D#" => "Eb",
       "E#" => "F",
       "Fb" => "E",
       "F#" => "Gb",
       "G#" => "Ab",
       "A#" => "Bb",
       "B#" => "C",
       _ => key
   }
   .to_string()
}

fn relative_key(key: &str, semitones: isize) -> String {
    let parts: Vec<&str> = key.split_whitespace().collect();
    if parts.len() != 2 {
        return key.to_string(); // fallback
    }

    let key_name = find_enharmonic_spelling(parts[0]);
    let key_mode = parts[1];
    let from_index = CHROMATIC_SCALE.iter().position(|&x| x == key_name).unwrap_or(0);

    let steps = if semitones < 0 {
        12 - ((-semitones) as usize % 12)
    } else {
        semitones as usize % 12
    };

    let new_index = (from_index + steps) % 12;
    let new_key = CHROMATIC_SCALE[new_index].to_string();

    format!("{} {}", new_key, key_mode)
}

pub fn transpose_song(song: Song, semitones: isize) -> Song {
    let mut transposed_blocks = Vec::new();

    for block in song.blocks {
        let mut new_lines = Vec::new();

        for line in block.lines {
            let mut new_segments = Vec::new();

            for segment in line.segments {
                match segment {
                    Segment::Measure(elements) => {
                        let new_elements = elements
                            .into_iter()
                            .map(|e| match e {
                                ChordOrText::Chord(c) => ChordOrText::Chord(relative_transpose(c, semitones)),
                                ChordOrText::Text(t) => ChordOrText::Text(t),
                            })
                            .collect();

                        new_segments.push(Segment::Measure(new_elements));
                    }
                    Segment::Inline(elements) => {
                        let new_elements = elements
                            .into_iter()
                            .map(|e| match e {
                                ChordOrText::Chord(c) => ChordOrText::Chord(relative_transpose(c, semitones)),
                                ChordOrText::Text(t) => ChordOrText::Text(t),
                            })
                            .collect();

                        new_segments.push(Segment::Inline(new_elements));
                    }
                }
            }

            new_lines.push(LyricLine {
                segments: new_segments,
            });
        }

        transposed_blocks.push(Block {
            section_name: block.section_name,
            lines: new_lines,
        });
    }

    let mut new_directives = song.directives.clone();
    new_directives.insert("key".to_string(), relative_key(song.directives.get("key").unwrap(), semitones));

    Song {
        directives: new_directives,
        blocks: transposed_blocks,
    }
}