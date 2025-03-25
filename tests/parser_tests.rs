use fs::read_to_string;
use std::collections::HashMap;
use std::fs;
use pest::Parser;
use leadsheetml::parser::*;
use leadsheetml::ast::*;
use leadsheetml::markdown::*;

#[test]
fn test_parses_simple_line(){
    let input = "[C]Hello, [G]world!";
    let parsed = LeadSheetMLParser::parse(Rule::lyric_line, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_measure(){
    let input = "| [C]Hello, [G]world! |";
    let parsed = LeadSheetMLParser::parse(Rule::measure, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_chord(){
    let input = "[C]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_complex_chords(){
    let input = "[C/G]";
    let input2 = "[C#maj7b5]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    let parsed2 = LeadSheetMLParser::parse(Rule::chord_token, input2);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    if let Err(e) = parsed2 {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    assert!(parsed2.is_ok());
}

#[test]
fn test_parses_simple_text(){
    let input = "Hello, world!";
    let parsed = LeadSheetMLParser::parse(Rule::text_token, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_directive(){
    let input = "@title: Hello, world!";
    let parsed = LeadSheetMLParser::parse(Rule::directive, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_simple_note(){
    let input = "A";
    let parsed = LeadSheetMLParser::parse(Rule::note, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_multiple_directives(){
    let input = "@title: For Absent Friends\n@artist: Genesis\n@key A Minor\n@time 4/4\n@tempo Andante";
    let parsed= LeadSheetMLParser::parse(Rule::directive_list, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_section_header(){
    let input = "#Title";
    let parsed = LeadSheetMLParser::parse(Rule::section_header, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_simple_block(){
    let input = "#Intro\n| [C]Hello, [G]world! |";
    let parsed = LeadSheetMLParser::parse(Rule::block, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
}

#[test]
fn test_parses_lyric_line(){
    let input = "[C]Hello, [G]world!";
    let input2 = "| [C]Hello, [G]world! |";
    let parsed = LeadSheetMLParser::parse(Rule::lyric_line, input);
    let parsed2 = LeadSheetMLParser::parse(Rule::lyric_line, input2);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    if let Err(e) = parsed2 {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    assert!(parsed2.is_ok());
}

#[test]
fn test_parses_complex_block(){
    let input = "#Verse\n[D] Sunday at [D/C#] six when they [D/C] close both the gates\n[D] A [Em] wi [D] dowed [Em]pair\n[D]Still [Em]sit[D]ting [A7]there,\n[G]Wonder [Em]if they're [A]late for [D]church\nAnd its [D/C#]cold, so they [D/C]fasten their coats\n[D]And [Em]cross [D]the [Em]grass, [D]theyre [Em]al[D]ways [A7]last.";
    let parsed = LeadSheetMLParser::parse(Rule::block, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
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
    let input = read_to_string("SongBook/Genesis/for_absent_friends.lmpl").unwrap();
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
    let parsed = parse_chord_token(parsed.unwrap().next().unwrap());
    assert_eq!(parsed, Chord {
        root: Note {
            letter: NoteLetter::C,
            accidental: Accidental::None
        },
        quality: None,
        extensions: Vec::new(),
        bass: None
    })
}

#[test]
fn test_parse_complex_chords_to_ast(){
    let input = "[C/G]";
    let input2 = "[C#maj7b5]";
    let parsed = LeadSheetMLParser::parse(Rule::chord_token, input);
    let parsed2 = LeadSheetMLParser::parse(Rule::chord_token, input2);
    assert!(parsed.is_ok());
    assert!(parsed2.is_ok());
    let parsed_chord = parse_chord_token(parsed.unwrap().next().unwrap());
    let parsed_chord2 = parse_chord_token(parsed2.unwrap().next().unwrap());
    assert_eq!(parsed_chord, Chord {
        root: Note {
        letter: NoteLetter::C,
        accidental: Accidental::None
        },
       quality: None,
        extensions:Vec::new(),
        bass: Some(Note
        { letter: NoteLetter::G,
            accidental: Accidental::None
        }
        )
    });
    assert_eq!(parsed_chord2, Chord{
        root: Note {
            letter: NoteLetter::C,
            accidental: Accidental::Sharp
        },
        quality: Some("maj".to_string()),
        extensions: vec![Some("7".to_string()),Some("b5".to_string())],
        bass: None
    });
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
    let input = "@title: For Absent Friends\n@artist: Genesis\n@key: A Minor\n@time: 4/4\n@tempo: Andante";
    let parsed= LeadSheetMLParser::parse(Rule::directive_list, input);
    assert!(parsed.is_ok());
    let mut directives:HashMap<String, String> = HashMap::new();
    for directive in parsed.unwrap().next().unwrap().into_inner() {
        let parsed_directive = parse_directive(directive);
        directives.insert(parsed_directive.name, parsed_directive.value);
    }
    assert_eq!(directives.get("title").unwrap(), "For Absent Friends");
    assert_eq!(directives.get("artist").unwrap(), "Genesis");
    assert_eq!(directives.get("key").unwrap(), "A Minor");
    assert_eq!(directives.get("time").unwrap(), "4/4");
    assert_eq!(directives.get("tempo").unwrap(), "Andante");
}

#[test]
fn test_parses_simple_section_header_to_ast(){
    let input = "#Title";
    let parsed = LeadSheetMLParser::parse(Rule::section_header, input);
    assert!(parsed.is_ok());
    assert_eq!("#Title", parsed.unwrap().next().unwrap().as_str());
}

#[test]
fn test_parses_simple_block_to_ast(){
    let input = "#Intro\n| Hello, world! |";
    let parsed = LeadSheetMLParser::parse(Rule::block, input);
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
    let input = "@title: Hello, Word\n#Intro\n| [C]Hello, [F]world! |";
    let input2 = "@title: Hello, Word\n#Intro\n[C]Hello, [F]world!";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    let parsed2 = LeadSheetMLParser::parse(Rule::song, input2);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    if let Err(e) = parsed2 {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    parse_song(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parse_song_to_ast(){
    let input = "@title: Twinkle Twinkle Little Star\n@key: C Major\n#Verse\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n\n#Chorus\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n\n#Bridge\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\nxw\n#Outro\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.";
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
    let input = read_to_string("SongBook/Genesis/for_absent_friends.lmpl").unwrap();
    let parsed = LeadSheetMLParser::parse(Rule::song, &input.as_str().trim());
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    parse_song(parsed.unwrap().next().unwrap());
}

#[test]
fn test_parse_simple_song_to_ml(){
    let input = "@title: Hello, Word\n#Intro\n| [C]Hello, world! |";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    let song = parse_song(parsed.unwrap().next().unwrap());
    println!("{}", render_song(&song))
}

#[test]
fn test_parse_song_to_ml(){
    let input = "@title: Twinkle Twinkle Little Star\n@key: C Major\n#Verse\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n\n#Chorus\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n\n#Bridge\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.\n\n#Outro\n[C]Twinkle, twinkle, little star\n[G]How I wonder what you are!\n[C]Up above the world so high\n[G]Like a diamond in the sky.";
    let parsed = LeadSheetMLParser::parse(Rule::song, input);
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    let song = parse_song(parsed.unwrap().next().unwrap());
    println!("{}", render_song(&song))
}

#[test]
fn test_parses_for_absent_friends_to_ml(){
    let input = read_to_string("SongBook/Genesis/for_absent_friends.lmpl").unwrap();
    let parsed = LeadSheetMLParser::parse(Rule::song, &input.as_str().trim());
    if let Err(e) = parsed {
        println!("Error: {}", e);
        panic!();
    }
    assert!(parsed.is_ok());
    let song = parse_song(parsed.unwrap().next().unwrap());
    println!("{}", render_song(&song))
}