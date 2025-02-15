use kanjivoci_lib;
use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided as an argument
    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-toml-file>", args[0]);
    }
    // Ensure at least one file is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file1> [file2] ...", args[0]);
        return Err("No path given".into()); // Exit the program with an error code
    }
    let mut carddeck = kanjivoci_lib::Carddecks::new();

    // Iterate over all files except the last one to check if they exist
    for file in &args[1..args.len() - 1] {
        if !std::path::Path::new(file).exists() {
            eprintln!("Error: {} does not exist", file);
            return Err("No path given".into()); // Exit the program with an error code
        } else {
            carddeck.read_vocab_toml(file)?;
        }
    }

    // Handle the last file (create if it doesn't exist)
    let last_file = &args[args.len() - 1];
    if !std::path::Path::new(last_file).exists() {
        // Create the file if it doesn't exist
        File::create(last_file).expect("Failed to create file");
        println!("Created empty file: {}", last_file);
    }
    // Get the file path from the arguments

    carddeck.read_vocab_toml(last_file)?;

    loop {
        print!("Press 'a' or 'あ' to run an iteration or 'e' to exit: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is shown

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "a" | "あ" => {
                println!("Adding an entry, please enter the Kanji word");
                let mut kanji = String::new();
                let mut readings = Vec::new();
                let mut meaning = String::new();
                io::stdin().read_line(&mut kanji).unwrap();
                let kanji = kanji.trim().to_string();
                if carddeck.vocab_find_duplicate(&kanji) {
                    continue;
                }
                let mut kanji_count = 0;
                let mut kana_count = 0;
                for c in kanji.chars() {
                    if is_kanji(c) {
                        println!("Please provide the reading for the Kanji: {}", c);

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        let input = input.trim().to_string();
                        for c in input.chars() {
                            if !is_kana(c) {
                                println!("U sure?");
                            }
                        }
                        readings.push(input);
                        kanji_count += 1;
                    }
                    if is_kana(c) {
                        kana_count += 1;
                    }
                }
                println!("Meaning?");
                io::stdin().read_line(&mut meaning).unwrap();
                let meaning = meaning.trim().to_string();
                carddeck.add_vocabcard(&kanji, &readings, &meaning);
                println!("kanjis: {} Kanas: {}", kanji_count, kana_count);
            }
            "e" => {
                println!("Exiting the loop.");
                break; // Exit the loop
            }
            _ => {
                println!("Invalid input. Please press 'a' or 'e'.");
            }
        }
    }
    carddeck.write_vocab_toml(last_file)?;

    carddeck.read_vocab_toml(last_file)?;

    carddeck.print_decks();
    Ok(())
}

// Function to check if a character is a Kanji (CJK Ideograph)
fn is_kanji(c: char) -> bool {
    // Kanji characters are generally in the Unicode range 0x4E00..0x9FFF
    c >= '\u{4E00}' && c <= '\u{9FFF}'
}

// Function to check if a character is Kana (Hiragana or Katakana)
fn is_kana(c: char) -> bool {
    // Hiragana characters are in the range 0x3040..0x309F
    // Katakana characters are in the range 0x30A0..0x30FF
    (c >= '\u{3040}' && c <= '\u{309F}') || (c >= '\u{30A0}' && c <= '\u{30FF}')
}
