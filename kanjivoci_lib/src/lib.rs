#[derive(Debug)]
pub struct Kanjicard {
    pub kanji: String,
    pub on_reading: Vec<String>,
    pub kun_reading: Vec<String>,
    pub meaning: String,
    pub level: i8, // N5 -> 5 till N1 -> 1. If 0: Not set yet
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

#[derive(Debug)]
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

#[derive(Debug)]
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
        self.kanjicards
            .push(Kanjicard::new(kanji, on_reading, kun_reading, level, meaning));
    }

    pub fn add_vocabcard(&mut self, kanji: &str, spelling: &Vec<String> , meaning: &str) {
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

    pub fn print_decks(self) {
        println!("kanjivocis");
        for card in self.kanjicards.iter() {
            println!("{}", card);
        }
        println!("Vocabcards");
        for card in self.vocabcards.iter() {
            println!("{}", card);
        }
    }
}
