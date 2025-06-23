use clap::{Arg, Command};
use colored::*;
use std::io::{self, Write};

mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize CLI with clap
    let matches = Command::new("lyrics-dsl")
        .version("0.1.0")
        .author("Your Name")
        .about("A domain-specific language for lyrics processing")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input lyrics file to process")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file for processed lyrics")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Enable verbose output")
        )
        .get_matches();

    // Print welcome message
    println!("{}", "🎵 Lyrics DSL Processor v0.1.0".bright_cyan().bold());
    println!("{}", "================================".bright_cyan());

    // Handle verbose flag
    let verbose = matches.get_flag("verbose");
    if verbose {
        println!("{}", "Verbose mode enabled".yellow());
    }

    // Test basic functionality
    test_dependencies(verbose)?;
    
    // Handle input/output arguments
    match (matches.get_one::<String>("input"), matches.get_one::<String>("output")) {
        (Some(input_file), output_file) => {
            process_lyrics_file(input_file, output_file.map(|s| s.as_str()), verbose)?;
        }
        (None, _) => {
            println!("{}", "No input file specified. Running in interactive mode...".green());
            interactive_mode(verbose)?;
        }
    }

    println!("{}", "\n✅ Lyrics DSL execution completed successfully!".bright_green().bold());
    Ok(())
}

fn test_dependencies(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("{}", "\n🔧 Testing dependencies...".blue());
    }

    // Test pest parsing capabilities
    test_pest_integration(verbose)?;
    
    // Test serde serialization
    test_serde_integration(verbose)?;
    
    // Test regex functionality
    test_regex_integration(verbose)?;
    
    if verbose {
        println!("{}", "✅ All dependencies working correctly".green());
    }
    
    Ok(())
}

fn test_pest_integration(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("  - Testing pest parser integration...");
    }

    // Parse a small DSL snippet using the pest grammar
    let sample = "title:\"Test\"\nartist:\"Author\"\nVERSE[1]\nLine one\nCHORUS\nLa la la\n";
    parser::parse_lyrics(sample)?;

    if verbose {
        println!("    ✓ Pest parser ready");
    }
    
    Ok(())
}

fn test_serde_integration(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("  - Testing serde serialization...");
    }
    
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug)]
    struct TestLyrics {
        title: String,
        artist: String,
        lines: Vec<String>,
    }
    
    let test_lyrics = TestLyrics {
        title: "Test Song".to_string(),
        artist: "Test Artist".to_string(),
        lines: vec!["Line 1".to_string(), "Line 2".to_string()],
    };
    
    let json = serde_json::to_string_pretty(&test_lyrics)?;
    if verbose {
        println!("    ✓ Serde serialization working");
        println!("    Sample JSON: {}", json.lines().next().unwrap_or(""));
    }
    
    Ok(())
}

fn test_regex_integration(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("  - Testing regex functionality...");
    }
    
    use regex::Regex;
    
    let verse_pattern = Regex::new(r"^VERSE\[\d+\]")?;
    let test_line = "VERSE[1]";
    
    if verse_pattern.is_match(test_line) {
        if verbose {
            println!("    ✓ Regex pattern matching working");
        }
    }
    
    Ok(())
}

fn process_lyrics_file(
    input_file: &str, 
    output_file: Option<&str>, 
    verbose: bool
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", format!("📄 Processing lyrics file: {}", input_file).cyan());
    
    // Check if input file exists
    if !std::path::Path::new(input_file).exists() {
        return Err(format!("Input file '{}' not found", input_file).into());
    }
    
    // Read the input file
    let content = std::fs::read_to_string(input_file)?;
    if verbose {
        println!("  - Read {} characters from input file", content.len());
    }
    
    // Basic processing (placeholder)
    let processed = format!("Processed content from {}:\n{}", input_file, content);
    
    // Handle output
    match output_file {
        Some(output_path) => {
            std::fs::write(output_path, &processed)?;
            println!("{}", format!("💾 Output written to: {}", output_path).green());
        }
        None => {
            println!("{}", "📺 Processed output:".yellow());
            println!("{}", processed);
        }
    }
    
    Ok(())
}

fn interactive_mode(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🎤 Interactive Lyrics DSL Mode".magenta().bold());
    println!("{}", "Type lyrics or DSL commands (type 'quit' to exit):".dimmed());
    
    loop {
        print!("{}", "lyrics> ".bright_blue());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "quit" || input == "exit" {
            println!("{}", "👋 Goodbye!".bright_yellow());
            break;
        }
        
        // Process the input (placeholder)
        process_interactive_input(input, verbose)?;
    }
    
    Ok(())
}

fn process_interactive_input(input: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("  - Processing input: '{}'", input);
    }
    
    // Basic pattern matching for different types of input
    use regex::Regex;
    
    let verse_regex = Regex::new(r"^VERSE\[\d+\]")?;
    let chorus_regex = Regex::new(r"^CHORUS$")?;
    let bridge_regex = Regex::new(r"^BRIDGE$")?;
    
    if verse_regex.is_match(input) {
        println!("{}", "🎼 Detected verse marker".blue());
    } else if chorus_regex.is_match(input) {
        println!("{}", "🎵 Detected chorus marker".green());
    } else if bridge_regex.is_match(input) {
        println!("{}", "🌉 Detected bridge marker".purple());
    } else if input.starts_with('[') && input.ends_with(']') {
        println!("{}", "🏷️  Detected custom section marker".yellow());
    } else {
        println!("{}", "📝 Processed lyric line".dimmed());
    }
    
    // Echo the processed result
    println!("   → {}", input.bright_white());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_integration() {
        assert!(test_dependencies(false).is_ok());
    }
    
    #[test]
    fn test_regex_patterns() {
        use regex::Regex;
        
        let verse_pattern = Regex::new(r"^VERSE\[\d+\]").unwrap();
        assert!(verse_pattern.is_match("VERSE[1]"));
        assert!(verse_pattern.is_match("VERSE[2]"));
        assert!(!verse_pattern.is_match("CHORUS"));
    }
    
    #[test]
    fn test_serde_functionality() {
        use serde::{Deserialize, Serialize};
        
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct TestStruct {
            name: String,
            value: i32,
        }
        
        let original = TestStruct {
            name: "test".to_string(),
            value: 42,
        };
        
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        
        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.value, deserialized.value);
    }
}
