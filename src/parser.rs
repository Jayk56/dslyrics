use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lyrics.pest"]
pub struct LyricsParser;

pub fn parse_lyrics(input: &str) -> Result<(), pest::error::Error<Rule>> {
    LyricsParser::parse(Rule::song, input).map(|_| ())
}

