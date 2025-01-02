# Naive Bayes

This is a Naive Bayes model written in rust that reads the text from PDF files.

## Training

To train the model, you need a `.csv` file where each line contains the path to the PDF file and a class that is corret for this file.
Seperate those two with a commma.
Example:
```csv
/path/to/a/file,class
```

Then run this command:
```bash
cargo run path/to/pdf_files.csv
```
