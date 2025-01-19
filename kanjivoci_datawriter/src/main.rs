use kanjivoci_lib;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut carddeck = kanjivoci_lib::Carddecks::new();

    let mut on_readings = Vec::new();
    on_readings.push("ジン".to_string());
    on_readings.push("ニン".to_string());
    let mut kun_readings = Vec::new();
    kun_readings.push("ひと".to_string());
    kun_readings.push("びと".to_string());
    kun_readings.push("り".to_string());
    kun_readings.push("と".to_string());
    carddeck.add_kanjicard("人", &on_readings, &kun_readings, 5, "person");

    let mut readings = Vec::new();
    readings.push("こい".to_string());
    readings.push("びと".to_string());

    carddeck.add_vocabcard("恋人", &readings, "loved one");

    let mut readings = Vec::new();
    readings.push("ゆう".to_string());
    readings.push("じん".to_string());

    carddeck.add_vocabcard("友人", &readings, "friend (formal)");

    loop {
        print!("Press 'a' to run an iteration or 'e' to exit: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is shown

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "a" => {
                println!("Adding an entry, please enter the Kanji word");
                let mut kanji = String::new();
                let mut readings = Vec::new();
                let mut meaning = String::new();
                io::stdin().read_line(&mut kanji).unwrap();
                let kanji = kanji.trim().to_string();
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
    carddeck.write_vocab_toml("vocab.toml")?;

    carddeck.read_vocab_toml("vocab.toml")?;

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
