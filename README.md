# Naive Bayes

This is a Naive Bayes model written in rust that reads the text from PDF files.

## Training

To train the model, prepare a `.csv` file where each line specifies the path to a PDF file and its corresponding class label.
Separate the file path and class label with a comma.

Example:
```csv
/path/to/a/file,class
```

Run the following command to start training:
```bash
cargo run path/to/pdf_files.csv
```
Ensure that the paths in the CSV file are correct and accessible to the program.
