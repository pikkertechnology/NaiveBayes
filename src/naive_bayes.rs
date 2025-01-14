use super::utils::extract_file_content;
use actix_multipart::form::tempfile::TempFile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct NaiveBayesModel {
    // Prior probabilities for each class
    class_priors: HashMap<String, f64>,

    // Conditional probabilities: class -> feature -> probability
    feature_probabilities: HashMap<String, HashMap<String, f64>>,

    // Optional: store feature counts for potential updates
    feature_counts: HashMap<String, HashMap<String, u32>>,

    // Metadata
    total_samples: u32,
    classes: Vec<String>,
    vocabulary: Vec<String>,
}

impl NaiveBayesModel {
    pub fn new() -> Self {
        if Path::new("model.bin").exists() {
            if let Ok(model) = Self::load_from_file("model.bin") {
                println!("Loaded model from binary file");
                return model;
            }
        }

        if Path::new("model.json").exists() {
            if let Ok(model) = Self::load_from_json("model.json") {
                println!("Loaded model from JSON file");
                return model;
            }
        }

        println!("Creating new empty model");
        NaiveBayesModel {
            class_priors: HashMap::new(),
            feature_probabilities: HashMap::new(),
            feature_counts: HashMap::new(),
            total_samples: 0,
            classes: Vec::new(),
            vocabulary: Vec::new(),
        }
    }

    fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        bincode::serialize_into(writer, self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    fn load_from_file(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        bincode::deserialize_from(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    fn save_to_json(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    fn load_from_json(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn train(&mut self, file: TempFile, class: String) -> Result<String, String> {
        let file_content = extract_file_content(&file);
        return file_content;
    }
}
