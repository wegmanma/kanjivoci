use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kanjicard {
    pub kanji: String,
    pub on_reading: Vec<String>,
    pub kun_reading: Vec<String>,
    pub meaning: String,
    pub level: i8,  // N5 -> 5 till N1 -> 1. If 0: Not set yet
    pub score: f32, // the higher, the better known
}

impl Kanjicard {
    pub fn new(
        kanji: &str,
        on_reading: &Vec<String>,
        kun_reading: &Vec<String>,
        level: i8,
        meaning: &str,
    ) -> Self {
        Kanjicard {
            kanji: kanji.to_string(),
            on_reading: on_reading.clone(),
            kun_reading: kun_reading.clone(),
            meaning: meaning.to_string(),
            level: level,
            score: 10.0, // Default score
        }
    }

    pub fn update_score(&mut self, delta: f32) {
        self.score += delta;
        if self.score < 0.0 {
            self.score = 0.0;
        }
    }
}

impl std::fmt::Display for Kanjicard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Kanji: {}, On Reading: {:?}, Kun Reading: {:?}, Meaning: {}, Level: {}, Score: {}",
            self.kanji, self.on_reading, self.kun_reading, self.meaning, self.level, self.score
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vocabcard {
    pub kanji: String,
    pub spelling: Vec<String>,
    pub meaning: String,
    pub score: f32, // the higher, the better known
}

impl Vocabcard {
    pub fn new(kanji: &str, spelling: &Vec<String>, meaning: &str) -> Self {
        Vocabcard {
            kanji: kanji.to_string(),
            spelling: spelling.clone(),
            meaning: meaning.to_string(),
            score: 10.0, // Default score
        }
    }

    pub fn update_score(&mut self, delta: f32) {
        self.score += delta;
        if self.score < 0.0 {
            self.score = 0.0;
        }
    }
}

impl std::fmt::Display for Vocabcard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Kanji: {}, Spelling: {:?}, Meaning: {}, Score: {}",
            self.kanji, self.spelling, self.meaning, self.score
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Carddecks {
    kanjicards: Vec<Kanjicard>,
    vocabcards: Vec<Vocabcard>,
}

impl Carddecks {
    pub fn new() -> Self {
        Carddecks {
            kanjicards: Vec::new(),
            vocabcards: Vec::new(),
        }
    }

    pub fn add_kanjicard(
        &mut self,
        kanji: &str,
        on_reading: &Vec<String>,
        kun_reading: &Vec<String>,
        level: i8,
        meaning: &str,
    ) {
        self.kanjicards.push(Kanjicard::new(
            kanji,
            on_reading,
            kun_reading,
            level,
            meaning,
        ));
    }

    pub fn add_vocabcard(&mut self, kanji: &str, spelling: &Vec<String>, meaning: &str) {
        self.vocabcards
            .push(Vocabcard::new(kanji, spelling, meaning));
    }

    pub fn sort_decks(&mut self) {
        // Sort the flashcards by score (ascending, so lower scores come first)
        self.kanjicards.sort_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.vocabcards.sort_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn vocab_find_duplicate(&mut self, kanji: &str) -> bool {
        if self
            .vocabcards
            .iter_mut()
            .find(|x| x.kanji == kanji)
            .is_some()
        {
            return true;
        } else {
            return false;
        }
    }

    pub fn vocab_kanji_align_score(&mut self, kanji: &str, delta: f32) {
        if let Some(card) = self.vocabcards.iter_mut().find(|x| x.kanji == kanji) {
            card.update_score(delta);
        }
    }

    pub fn kanji_kanji_align_score(&mut self, kanji: &str, delta: f32) {
        if let Some(card) = self.kanjicards.iter_mut().find(|x| x.kanji == kanji) {
            card.update_score(delta);
        }
    }

    pub fn print_decks(&self) {
        println!("kanjivocis");
        for card in self.kanjicards.iter() {
            println!("{}", card);
        }
        println!("Vocabcards");
        for card in self.vocabcards.iter() {
            println!("{}", card);
        }
    }

    pub fn write_vocab_toml_bak(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Vocabcard getting written?");
        println!("Vocabcards");
        let mut file = File::create(path)?;
        let toml_header = "[[Vocabcard]]\n";
        for card in self.vocabcards.iter() {
            let mut toml_string = toml::to_string(card)?;
            toml_string.insert_str(0, toml_header);
            println!("{}", toml_string);
            file.write_all(toml_string.as_bytes())?;
        }
        Ok(())
    }

    pub fn write_vocab_toml(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Vocabcard getting written?");
        println!("Vocabcards");
        let mut file = File::create(path)?;
        let vocab_vec = self.vocabcards.clone();
        let vocab_map: HashMap<String, Vocabcard> = vocab_vec
            .into_iter()
            .map(|card| (card.kanji.clone(), card))
            .collect();
        let toml_string = toml::to_string(&vocab_map)?;
        println!("{}", toml_string);
        file.write_all(toml_string.as_bytes())?;
        Ok(())
    }

    pub fn read_vocab_toml(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)?;
        let vocab_map: HashMap<String, Vocabcard> = toml::de::from_str(&toml_string)?;
        println!("{:?}", vocab_map);
        for card in vocab_map.values() {
            if !self
                .vocabcards
                .iter()
                .any(|existing_card| existing_card.kanji == card.kanji)
            {
                self.vocabcards.push(card.clone());
            } else {
                println!("Duplicate found for word: {}", card.kanji);
            }
        }
        Ok(())
    }
}
