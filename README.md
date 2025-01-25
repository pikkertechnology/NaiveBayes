# Naive Bayes

This is a Naive Bayes classifier model written in rust that reads the text from a file. This project is part of [study material sharing website](https://openabi.ee) where it is used for classifying different categories of uploaded material.    

## How it works

The model has 2 endpoints: **train** and **predict**

We use endpoints so that the model can be used in a [***docker container***](https://www.docker.com/).

## Train

The train (POST) endpoint is ```localhost:8080/train```. In the request body there are 2 expected fields:
1. **file** - the file for training
2. **class** - the class that the file should be classified as

The response is either `Model successfully trained and data saved!` or an error message.

## Predict

The predict (POST) endpoint is ```localhost:8080/predict```. In the request body there is 1 expected field:
1. **file** - the file that's class is predicted

The response is either the predicted class or an error message.
