use core::f64;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use chrono;
use regex::Regex;

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
    fn get_model_dirs() -> std::io::Result<(PathBuf, PathBuf)> {
        let model_dir = PathBuf::from("model");
        let version_dir = PathBuf::from("model/versions");
        fs::create_dir_all(&model_dir)?;
        fs::create_dir_all(&version_dir)?;
        Ok((model_dir, version_dir))
    }

    fn get_default_paths(class: &str) -> std::io::Result<(PathBuf, PathBuf)> {
        let (model_dir, version_dir) = Self::get_model_dirs()?;
        let latest_path = model_dir.join("model.bin");

        let timestamp = chrono::offset::Utc::now().format("%Y-%m-%d--%H-%M-%S").to_string();
        let version_path = version_dir.join(format!("model--{}--{}.bin", class, timestamp));
        Ok((latest_path, version_path))
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
        let paths = Self::get_default_paths("_");

        match paths {
            Ok((latest_path, _)) => {
                if let Ok(model) = Self::load_from_file(&latest_path) {
                    info!(
                        "Loaded model from a binary with path: {}",
                        latest_path.display()
                    );
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

    pub fn save(&self, class: &str) -> std::io::Result<()> {
        let (latest_path, version_path) = Self::get_default_paths(class)?;
        self.save_to_file(&latest_path)?;
        self.save_to_file(&version_path)?;
        Ok(())
    }

    pub fn train(&mut self, file_content: String, class: String) -> Result<String, String> {
        info!("Text with class '{}' was given for training", class);

        debug!("Starting to collect data.");
        self.collect_data(&file_content, &class);
        debug!("Data collecting finished.");

        match self.save(&class) {
            Ok(_) => {
                info!("Model successfully trained and data saved!");
                Ok("Model successfully trained and data saved!".to_string())
            }
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

        let feature_counts = self
            .class_feature_counts
            .entry(class.to_string())
            .or_insert(HashMap::new());

        let re = Regex::new("[!?.,\"']").unwrap();

        for feature in text.split_whitespace() {
            let feature = re.replace_all(&feature.to_lowercase(), "").to_string();
            self.features.insert(feature.clone());
            *feature_counts.entry(feature).or_insert(0) += 1;
        }
    }

    pub fn predict(&self, text: String) -> Result<String, String> {
        info!("Text was given for predicting");

        let mut best_class = String::new();
        let mut best_probability = f64::NEG_INFINITY;

        debug!("Starting to predict class.");
        for class in &self.classes {
            let probability = self.calculate_probability(&text, &class);
            if probability > best_probability {
                best_class = class.to_string();
                best_probability = probability;
            }
        }
        debug!("Class predicting finished.");

        info!("The prediction for file was '{}'", best_class);
        return Ok(best_class);
    }

    fn calculate_probability(&self, text: &str, class: &str) -> f64 {
        let class_count = *self.class_counts.get(class).unwrap_or(&0);
        let prior_probability = (class_count as f64 / self.samples_count as f64).ln();

        let feature_counts = self.class_feature_counts.get(class).unwrap();
        let total_features_in_class: u32 = feature_counts.values().sum();
        let all_features_count = self.features.len() as u32;

        let re = Regex::new("[!?.,\"']").unwrap();

        let mut likelihood = 0.0;
        for feature in text.split_whitespace() {
            let feature = re.replace_all(&feature.to_lowercase(), "").to_string();
            let feature_count = *feature_counts.get(&feature).unwrap_or(&0);
            likelihood += ((feature_count + 1) as f64
                / (total_features_in_class + all_features_count) as f64)
                .ln();
        }

        return prior_probability + likelihood;
    }
}
