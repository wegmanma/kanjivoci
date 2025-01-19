use kanjivoci_lib;

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

    carddeck.vocab_kanji_align_score("人", -1.5);
    carddeck.kanji_kanji_align_score("人", 1.2);
    carddeck.sort_decks();
    carddeck.print_decks();

    carddeck.write_vocab_toml("vocab.toml")?;

    carddeck.read_vocab_toml("vocab.toml")?;

    carddeck.print_decks();
    Ok(())
}
