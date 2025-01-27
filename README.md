# Naive Bayes

This is a Naive Bayes classifier model written in rust. This project is part of [study material sharing website](https://openabi.ee) where it is used for classifying different categories of uploaded material.    

## How it works

The model has 2 endpoints: **train** and **predict**

We use endpoints so that the model can be used in a [***docker container***](https://www.docker.com/). The endpoints are realized with [Actix web](https://actix.rs/).

After each training the model saves it's data to a `model.bin` file in the `/model/` folder. Each training also makes a version file so the model can be rolled back.

## Train

The train (POST) endpoint is ```localhost:8080/train```. In the request body (JSON) there are 2 expected fields:
1. **text** - the text for training
2. **class** - the class that the text should be classified as

The response is either `Model successfully trained and data saved!` or an error message.

## Predict

The predict (POST) endpoint is ```localhost:8080/predict```. In the request body (JSON) there is 1 expected field:
1. **text** - the text that's class is predicted

The response is either the predicted class or an error message.
