use lyrics_dsl::parser::parse_lyrics;

#[test]
fn parse_basic_song() {
    let input = "title:\"My Song\"\nartist:Author\nVERSE[1]\nHello\nCHORUS\nWorld\n";
    assert!(parse_lyrics(input).is_ok());
}

#[test]
fn parse_failure() {
    // Missing newline before section should fail
    let bad_input = "title:Test artist:Author";
    assert!(parse_lyrics(bad_input).is_err());
}

#[test]
fn parse_glitch_song() {
    let song = std::fs::read_to_string("tests/glitch_song.txt").expect("read song");
    assert!(parse_lyrics(&song).is_ok());
}

