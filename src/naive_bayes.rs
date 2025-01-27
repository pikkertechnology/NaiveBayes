use super::utils::extract_file_content;
use actix_multipart::form::tempfile::TempFile;
use log::{info, warn, error, debug};
use serde::{Deserialize, Serialize};
use core::f64;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct NaiveBayesModel {
    // class -> feature -> count
    class_feature_counts: HashMap<String, HashMap<String, u32>>,
    // class -> count
    class_counts: HashMap<String, u32>,

    // all classes
    classes: HashSet<String>,
    // all features
    features: HashSet<String>,
    // count of samples
    samples_count: u32,
}

impl NaiveBayesModel {
    fn get_model_dir() -> std::io::Result<PathBuf> {
        let model_dir = PathBuf::from("model");
        fs::create_dir_all(&model_dir)?;
        Ok(model_dir)
    }

    fn get_default_paths() -> std::io::Result<PathBuf> {
        let model_dir = Self::get_model_dir()?;
        let bin_path = model_dir.join("model.bin");
        Ok(bin_path)
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

    pub fn new() -> Self {
        let paths = Self::get_default_paths();
        
        match paths {
            Ok(bin_path) => {
                if let Ok(model) = Self::load_from_file(&bin_path) {
                    info!("Loaded model from a binary with path: {}", bin_path.display());
                    return model;
                }
            }
            Err(e) => {
                warn!("Could not determine model directory: {e}");
            }
        }

        info!("Creating a new empty model.");
        NaiveBayesModel {
            class_feature_counts: HashMap::new(),
            class_counts: HashMap::new(),
            classes: HashSet::new(),
            features: HashSet::new(),
            samples_count: 0,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let bin_path = Self::get_default_paths()?;
        self.save_to_file(&bin_path)?;
        Ok(())
    }

    pub fn train(&mut self, file: TempFile, class: String) -> Result<String, String> {
        let file_content = match extract_file_content(&file) {
            Ok(content) => content,
            Err(err) => {
                error!("Could not extract the file's content: {err}");
                return Err(err.to_string())
            }
        };
        debug!("Successfully extracted file's content.");

        info!("File '{}' with class '{}' was given for training", file.file_name.unwrap(), class);

        debug!("Starting to collect data.");
        self.collect_data(&file_content, &class);
        debug!("Data collecting finished.");

        match self.save() {
            Ok(_) => {
                info!("Model successfully trained and data saved!");
                Ok("Model successfully trained and data saved!".to_string())
            },
            Err(err) => {
                error!("Something went wrong with saving: {err}");
                Err(err.to_string())
            }
        }
    }

    fn collect_data(&mut self, text: &str, class: &str) {
        self.samples_count += 1;

        self.classes.insert(class.to_string());
        *self.class_counts.entry(class.to_string()).or_insert(0) += 1;

        let feature_counts = self.class_feature_counts.entry(class.to_string()).or_insert(HashMap::new());

        for feature in text.split_whitespace() {
            let feature = feature.to_lowercase();
            self.features.insert(feature.clone());
            *feature_counts.entry(feature).or_insert(0) += 1;
        }
    }

    pub fn predict(&self, file: TempFile) -> Result<String, String> {
        let file_content = match extract_file_content(&file) {
            Ok(content) => content,
            Err(err) => {
                error!("Could not extract the file's content: {err}");
                return Err(err.to_string())
            }
        };
        debug!("Successfully extracted file's content.");

        info!("File '{}' was given for predicting", file.file_name.clone().unwrap());

        let mut best_class = String::new();
        let mut best_probability = f64::NEG_INFINITY;

        debug!("Starting to predict class.");
        for class in &self.classes {
            let probability = self.calculate_probability(&file_content, &class);
            if probability > best_probability {
                best_class = class.to_string();
                best_probability = probability;
            }
        }
        debug!("Class predicting finished.");

        info!("The prediction for file '{}' was '{}'", file.file_name.unwrap(), best_class);
        return Ok(best_class)
    }

    fn calculate_probability(&self, text: &str, class: &str) -> f64 {
        let class_count = *self.class_counts.get(class).unwrap_or(&0);
        let prior_probability = (class_count as f64 / self.samples_count as f64).ln();

        let feature_counts = self.class_feature_counts.get(class).unwrap();
        let total_features_in_class: u32 = feature_counts.values().sum();
        let all_features_count = self.features.len() as u32;

        let mut likelihood = 0.0;
        for feature in text.split_whitespace() {
            let feature = feature.to_lowercase();
            let feature_count = *feature_counts.get(&feature).unwrap_or(&0);
            likelihood += ((feature_count + 1) as f64 / (total_features_in_class + all_features_count) as f64).ln();
        }

        return prior_probability + likelihood
    }
}
