# Naive Bayes

This is a Naive Bayes classifier model written in rust that reads the text from a file. This project is part of [study material sharing website](https://openabi.ee) where it is used for classifying different categories of uploaded material.    

## How it works

The model has 2 endpoints: **train** and **predict**

We use endpoints so that the model can be used in a [***docker container***](https://www.docker.com/).

## Train

The train endpoint is ```localhost:8080/train```. In the body there are 2 expected fields:
1. **file** - the file itself
2. **class** - the class that the file should be classified as

The answer is either `Model successfully trained and data saved!` or an error message.

## Predict
