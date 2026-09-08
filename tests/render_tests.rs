use leadsheetml::ast::*;
use leadsheetml::parser::{parse_chord, LeadSheetMLParser, Rule};
use leadsheetml::render::{DefaultLeadSheetRenderer, LeadSheetRenderer};
use markup_engine::{HtmlEngine, MarkdownEngine};
use pest::Parser;
use std::collections::HashMap;

fn chord(label: &str) -> ChordOrText {
    ChordOrText::Chord(
        parse_chord(
            LeadSheetMLParser::parse(Rule::chord, label)
                .unwrap()
                .next()
                .unwrap(),
        )
        .unwrap(),
    )
}

fn text(value: &str) -> ChordOrText {
    ChordOrText::Text(value.into())
}

fn song(segments: Vec<Segment>) -> Song {
    Song {
        directives: HashMap::new(),
        blocks: vec![Block {
            section_name: "Verse".into(),
            lines: vec![LyricLine { segments }],
        }],
    }
}

fn markdown(items: Vec<ChordOrText>) -> String {
    DefaultLeadSheetRenderer.render_song(&MarkdownEngine, &song(vec![Segment::Inline(items)]))
}

#[test]
fn chords_anchor_to_lyrics_without_adding_spaces() {
    assert_eq!(
        markdown(vec![chord("C"), text("Hello "), chord("G"), text("world")]),
        "### Verse\n```\nC     G\nHello world\n```\n"
    );
}

#[test]
fn collisions_shift_lyrics_and_subsequent_chords_together() {
    assert_eq!(
        markdown(vec![
            chord("Cmaj7"),
            text("Hi"),
            chord("G"),
            text("there "),
            chord("Am"),
            text("friend")
        ]),
        "### Verse\n```\nCmaj7 G     Am\nHi    there friend\n```\n"
    );
}

#[test]
fn adjacent_and_trailing_chords_keep_their_order() {
    assert_eq!(
        markdown(vec![
            chord("C"),
            chord("G"),
            text("Hi"),
            chord("Am"),
            chord("F")
        ]),
        "### Verse\n```\nC G Am F\n  Hi   \n```\n"
    );
}

#[test]
fn newlines_reset_anchors_and_segments_share_positions() {
    let input = song(vec![
        Segment::Inline(vec![text("éé "), chord("C"), text("hi\r\n")]),
        Segment::Measure(vec![chord("G"), text("go")]),
        Segment::Inline(vec![text(" on"), chord("Am"), text("!\n\n")]),
    ]);
    assert_eq!(
        DefaultLeadSheetRenderer.render_song(&MarkdownEngine, &input),
        "### Verse\n```\n   C\néé hi\nG    Am\ngo on!\n```\n"
    );
}

#[test]
fn html_escapes_after_layout_and_bolds_chords() {
    let mut input = song(vec![Segment::Inline(vec![
        text("<&"),
        chord("C"),
        text("word"),
    ])]);
    input.directives.insert("title".into(), "<Title>".into());
    input.directives.insert("artist".into(), "A & B".into());
    assert_eq!(DefaultLeadSheetRenderer.render_song(&HtmlEngine, &input),
        "<h1>&lt;Title&gt;</h1>\n<br/><i>A &amp; B</i><br/><h3>Verse</h3>\n<br/><pre><b>  C</b>\n&lt;&amp;word</pre>\n<br/>");
}

#[test]
fn markdown_preserves_literal_formatting_and_protects_fences() {
    assert_eq!(
        markdown(vec![text("**words**\n```\n")]),
        "### Verse\n````\n\n**words**\n\n```\n````\n"
    );
}

#[test]
fn tabs_expand_consistently_before_anchoring() {
    assert_eq!(
        markdown(vec![text("a\t"), chord("C"), text("b")]),
        "### Verse\n```\n    C\na   b\n```\n"
    );
}

#[test]
fn parsed_lines_ending_in_chords_stay_separate() {
    for newline in ["\n", "\r\n"] {
        for ending in ["[C]", "[C/B] [Am] [C]", "[C]   "] {
            for following in [
                "And touched the [G]sound of [Am]silence",
                "[G]And touched the sound",
            ] {
                let source = format!(
                    "#Verse 2{newline}That split the night{ending}{newline}{following}{newline}"
                );
                let parsed = leadsheetml::parser::parse_song_from_str(&source).unwrap();
                for engine in [
                    &MarkdownEngine as &dyn leadsheetml::render::LeadSheetFormat,
                    &HtmlEngine,
                ] {
                    let rendered = DefaultLeadSheetRenderer.render_song(engine, &parsed);
                    let lyric_lines: Vec<_> = rendered
                        .lines()
                        .filter(|line| line.contains("That split") || line.contains("And touched"))
                        .collect();
                    assert_eq!(lyric_lines.len(), 2, "{source:?}: {rendered}");
                    assert!(lyric_lines[0].contains("That split the night"));
                    assert!(lyric_lines[1].starts_with("And touched"), "{rendered}");
                }
            }
        }
    }
}
