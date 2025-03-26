use crate::ast::*;
use markdown::to_html;
use markup_engine::engine::MarkupEngine;


pub trait LeadSheetRenderer {
    fn render_song(&self, engine:&dyn MarkupEngine,song: &Song) -> String;

}

pub struct DefaultLeadSheetRenderer;
impl LeadSheetRenderer for DefaultLeadSheetRenderer {
    fn render_song(&self, engine: &dyn MarkupEngine, song: &Song) -> String {
        let mut output = String::new();

        if let Some(title) = song.directives.get("title") {
            output.push_str(&engine.header(1, title));
            output.push_str(&engine.linebreak())
        }
        if let Some(artist) = song.directives.get("artist") {
            output.push_str(&engine.italic(artist));
            output.push_str(&engine.linebreak())
        }

        for block in &song.blocks {
            let header_text = block.section_name.trim_start_matches('#');
            output.push_str(&engine.header(3, header_text));
            output.push_str(&engine.linebreak());

            for line in &block.lines {
                let mut chord_line = String::new();
                let mut lyric_line = String::new();
                for segment in &line.segments {
                    match segment {
                        Segment::Measure(items) | Segment::Inline(items) => {
                            for item in items {
                                match item {
                                    ChordOrText::Chord(c) => {
                                        let chord = format_chord(c.clone());
                                        chord_line.push_str(&format!("{:4}", chord));
                                        lyric_line.push_str(" ".repeat(chord.len()).as_str());
                                    }
                                    ChordOrText::Text(text) => {
                                        chord_line.push_str(&" ".repeat(text.len()));
                                        let arr:Vec<String> = simple_split(text, '\n');
                                        lyric_line.push_str(arr[0].as_str());

                                        if contains_newline(&text) {
                                            output.push_str(engine.line_segment(chord_line.as_str()).as_str());
                                            output.push_str(engine.linebreak().as_str());
                                            output.push_str(engine.line_segment(lyric_line.as_str()).as_str());
                                            output.push_str(engine.linebreak().as_str());
                                            chord_line = String::new();
                                            lyric_line = String::new();
                                        }
                                        if arr[1].len() > 0 {
                                            lyric_line.push_str(engine.line_segment(arr[1].as_str()).as_str());
                                            chord_line.push_str(engine.line_segment(" ".repeat(arr[1].len()).as_str()).as_str());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if !chord_line.trim().is_empty() || !lyric_line.trim().is_empty() {
                    output.push_str(engine.line_segment(chord_line.as_str()).as_str());
                    output.push_str(engine.linebreak().as_str());
                    output.push_str(engine.line_segment(lyric_line.as_str()).as_str());
                    output.push_str(engine.linebreak().as_str());
                }
            }
        }

        output
    }
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

pub fn wrap_markdown(front:String, string:String, mut back:String, newline:bool) -> String {
    if newline {
        back.push('\n');
    }
    (front + &*string + &*back).to_string()
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