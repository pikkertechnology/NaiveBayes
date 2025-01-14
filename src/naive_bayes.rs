use super::utils::extract_file_content;
use actix_multipart::form::tempfile::TempFile;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::intrinsics::mir::Len;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct NaiveBayesModel {
    // Prior probabilities for each class
    class_priors: HashMap<String, f64>,

    // Conditional probabilities: class -> feature -> probability
    feature_probabilities: HashMap<String, HashMap<String, f64>>,

    // Feature counts for potential updates
    feature_counts: HashMap<String, HashMap<String, u32>>,

    // Class sample counts
    class_counts: HashMap<String, u32>,

    // Metadata
    total_samples: u32,
    classes: HashSet<String>,
    vocabulary: HashSet<String>,
}

impl NaiveBayesModel {
    fn get_model_dir() -> std::io::Result<PathBuf> {
        let model_dir = PathBuf::from("model");
        fs::create_dir_all(&model_dir)?;
        Ok(model_dir)
    }

    fn get_default_paths() -> std::io::Result<(PathBuf, PathBuf)> {
        let model_dir = Self::get_model_dir()?;
        let bin_path = model_dir.join("model.bin");
        let json_path = model_dir.join("model.json");
        Ok((bin_path, json_path))
    }

    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        
        bincode::serialize_into(writer, self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
    
    fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        bincode::deserialize_from(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
    
    fn save_to_json<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        
        serde_json::to_writer_pretty(writer, self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
    
    fn load_from_json<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        serde_json::from_reader(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn new() -> Self {
        let paths = Self::get_default_paths();
        
        match paths {
            Ok((bin_path, json_path)) => {
                if bin_path.exists() {
                    if let Ok(model) = Self::load_from_file(&bin_path) {
                        println!("Loaded model from: {}", bin_path.display());
                        return model;
                    }
                }

                if json_path.exists() {
                    if let Ok(model) = Self::load_from_json(&json_path) {
                        println!("Loaded model from: {}", json_path.display());
                        return model;
                    }
                }
            }
            Err(e) => {
                println!("Warning: Could not determine model directory: {}", e);
            }
        }

        println!("Creating new empty model");
        NaiveBayesModel {
            class_priors: HashMap::new(),
            feature_probabilities: HashMap::new(),
            feature_counts: HashMap::new(),
            total_samples: 0,
            classes: HashSet::new(),
            vocabulary: HashSet::new(),
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let (bin_path, json_path) = Self::get_default_paths()?;
        self.save_to_file(&bin_path)?;
        self.save_to_json(&json_path)?;
        Ok(())
    }

    pub fn train(&mut self, file: TempFile, class: String) -> Result<String, String> {
        let file_content = extract_file_content(&file);
        let file_content = match file_content {
            Ok(content) => content,
            Err(_) => return file_content
        };

        self.collect_data(file_content, class);


        match self.save() {
            Ok(_) => Ok("Model successfully trained and data saved!".to_string()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn collect_data(&mut self, text: String, class: String) {
        self.total_samples += 1;
        self.classes.insert(class.clone());
        *self.class_counts.entry(class.clone()).or_default() += 1;

        let features = text.split(" ");

        for feature in features {

        }
    }
}
