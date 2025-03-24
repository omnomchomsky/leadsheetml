use std::fs;
use pest::Parser;
use lead_sheet_ml::*;

#[test]
fn test_parses_simple_line(){
    let input = "[C]Hello, [G]world!";
    let parsed = LeadSheetMLParser::parse(Rule::lyric_line, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_measure(){
    let input = "| [C]Hello, [G]world! |";
    let parsed = LeadSheetMLParser::parse(Rule::measure, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_chord(){
    let input = "[C]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_complex_chords(){
    let input = "[C/G]";
    let input2 = "[C#maj7b5]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    let parsed2 = LeadSheetMLParser::parse(Rule::chord_token, input2);
    assert!(parsed.is_ok());
    assert!(parsed2.is_ok());
}

#[test]
fn test_parses_simple_text(){
    let input = "Hello, world!";
    let parsed = LeadSheetMLParser::parse(Rule::text_token, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_directive(){
    let input = "@title: Hello, world!";
    let parsed = LeadSheetMLParser::parse(Rule::directive, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_simple_note(){
    let input = "A";
    let parsed = LeadSheetMLParser::parse(Rule::note, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_multiple_directives(){
    let input = "@title: For Absent Friends\n@artist: Genesis\n@key A Minor\n@time 4/4\n@tempo Andante";
    let parsed= LeadSheetMLParser::parse(Rule::directive_list, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_section_header(){
    let input = "#Title";
    let parsed = LeadSheetMLParser::parse(Rule::section_header, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_block(){
    let input = "#Intro\n| Hello, world! |";
    let parsed = LeadSheetMLParser::parse(Rule::lyric_block, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_complex_block(){
    let input = "#Verse\n[D] Sunday at [D/C#] six when they [D/C] close both the gates\n[D] A [Em] wi [D] dowed [Em]pair\n[D]Still [Em]sit[D]ting [A7]there,\n[G]Wonder [Em]if they're [A]late for [D]church\nAnd its [D/C#]cold, so they [D/C]fasten their coats\n[D]And [Em]cross [D]the [Em]grass, [D]theyre [Em]al[D]ways [A7]last.";
    let parsed = LeadSheetMLParser::parse(Rule::block, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_simple_song(){
    let input = "@title: Hello, Word\n#Intro\n| [C]Hello, world! |";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_song(){
    let input = "@title: Twinkle Twinkle Little Star\n@key: C Major\n#Verse\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Chorus\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Bridge\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Outro\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_for_absent_friends(){
    let input = fs::read_to_string("SongBook/Genesis/for_absent_friends.impl").unwrap();
    let parsed = LeadSheetMLParser::parse(Rule::song, &input.as_str().trim());
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_line_to_ast(){
    let input = "[C]Hello, [G]world!";
    let parsed = LeadSheetMLParser::parse(Rule::lyric_line, input);
    assert!(parsed.is_ok());
    parse_line(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parses_simple_measure_to_ast(){
    let input = "| [C]Hello, [G]world! |";
    let parsed = LeadSheetMLParser::parse(Rule::measure, input);
    assert!(parsed.is_ok());
    parse_measure(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parses_simple_chord_to_ast(){
    let input = "[C]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    assert!(parsed.is_ok());
    let parsed = parse_chord(parsed.unwrap().next().unwrap());
    /*assert_eq!(parsed, Chord {
        root: Note {
            letter: NoteLetter::C,
            accidental: Accidental::None
        },
        quality: None,
        extensions: Vec::new(),
        bass: None
    })*/
}

#[test]
fn test_parse_complex_chords_to_ast(){
    let input = "[C/G]";
    let input2 = "[C#maj7b5]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    let parsed2 = LeadSheetMLParser::parse(Rule::chord_token, input2);
    assert!(parsed.is_ok());
    assert!(parsed2.is_ok());
    parse_chord(parsed.unwrap().next().unwrap());
    parse_chord(parsed2.unwrap().next().unwrap());
}

#[test]
fn test_parses_simple_text_to_ast(){
    let input = "Hello, world!";
    let parsed = LeadSheetMLParser::parse(Rule::text_token, input);
    assert!(parsed.is_ok());
    parse_text_token(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parses_simple_directive_to_ast(){
    let input = "@title: Hello, world!";
    let parsed = LeadSheetMLParser::parse(Rule::directive, input);
    assert!(parsed.is_ok());
    parse_directive(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parse_simple_note_to_ast(){
    let input = "A";
    let parsed = LeadSheetMLParser::parse(Rule::note, input);
    assert!(parsed.is_ok());
    parse_note(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parse_multiple_directives_to_ast(){
    let input = "@title: For Absent Friends\n@artist: Genesis\n@key A Minor\n@time 4/4\n@tempo Andante";
    let parsed= LeadSheetMLParser::parse(Rule::directive_list, input);
    assert!(parsed.is_ok());
    for directive in parsed.unwrap().next().unwrap().into_inner() {
        parse_directive(directive);
    }
}

#[test]
fn test_parses_simple_section_header_to_ast(){
    let input = "#Title";
    let parsed = LeadSheetMLParser::parse(Rule::section_header, input);
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_block_to_ast(){
    let input = "#Intro\n| Hello, world! |";
    let parsed = LeadSheetMLParser::parse(Rule::lyric_block, input);
    parse_block(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parses_complex_block_to_ast(){
    let input = "#Verse\n[D] Sunday at [D/C#] six when they [D/C] close both the gates\n[D] A [Em] wi [D] dowed [Em]pair\n[D]Still [Em]sit[D]ting [A7]there,\n[G]Wonder [Em]if they're [A]late for [D]church\nAnd its [D/C#]cold, so they [D/C]fasten their coats\n[D]And [Em]cross [D]the [Em]grass, [D]theyre [Em]al[D]ways [A7]last.";
    let parsed = LeadSheetMLParser::parse(Rule::block, input);
    assert!(parsed.is_ok());
    parse_block(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parse_simple_song_to_ast(){
    let input = "@title: Hello, Word\n#Intro\n| [C]Hello, world! |";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    parse_song(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parse_song_to_ast(){
    let input = "@title: Twinkle Twinkle Little Star\n@key: C Major\n#Verse\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Chorus\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Bridge\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n#Outro\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    parse_song(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parses_for_absent_friends_to_ast(){
    let input = fs::read_to_string("SongBook/Genesis/for_absent_friends.impl").unwrap();
    let parsed = LeadSheetMLParser::parse(Rule::song, &input.as_str().trim());
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    parse_song(parsed.unwrap().next().unwrap());
}