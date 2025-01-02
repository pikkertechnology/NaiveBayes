use csv::Reader;
use std::{env, error::Error};

mod naive_bayes;
use naive_bayes::NaiveBayes;

#[derive(serde::Deserialize)]
struct PdfRecord {
    file_path: String,
    class: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let csv_path = args.get(1).ok_or("Usage: cargo run <csv_file_path>")?;

    let mut classifier = NaiveBayes::new();
    let mut rdr = Reader::from_path(csv_path)?;

    for result in rdr.deserialize() {
        let record: PdfRecord = result?;
        let text = extract_text_from_pdf(&record.file_path)?;
        let features = tokenize_text(&text);
        classifier.train(&features, &record.class);
    }

    Ok(())
}

fn extract_text_from_pdf(path: &str) -> Result<String, Box<dyn Error>> {
    let bytes = std::fs::read(path).unwrap();
    let text = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    Ok(text)
}

fn tokenize_text(text: &str) -> Vec<String> {
    text.split_whitespace().map(|s| s.to_lowercase()).collect()
}
