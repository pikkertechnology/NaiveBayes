use std::collections::HashMap;

#[derive(Default)]
pub struct NaiveBayes {
    word_counts: HashMap<String, HashMap<String, usize>>,
    subject_counts: HashMap<String, usize>,
    total_number_of_input: usize,
}

impl NaiveBayes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn train(&mut self, words: &[String], subject: &str) {
        self.total_number_of_input += 1;
        *self.subject_counts.entry(subject.to_string()).or_insert(0) += 1;

        for word in words {
            let subject_map = self.word_counts.entry(subject.to_string()).or_default();
            *subject_map.entry(word.to_string()).or_insert(0) += 1;
        }
    }
}
