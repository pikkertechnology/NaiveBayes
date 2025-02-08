# Naive Bayes

This is a Naive Bayes classifier model written in rust. This project is part of [study material sharing website](https://openabi.ee) where it is used for classifying different categories of uploaded material.

## How it works

The model has 2 endpoints: **train** and **predict**.

We use endpoints so that the model can be used in a [***docker container***](https://www.docker.com/). The endpoints are realized with [Actix web](https://actix.rs/).

The model saves it's self to a `.bin` file in a `/model` foler. Also each training is saved as a seperate version, so rolling a model back few versions is easy.

## Used technologies
 - Rust (2021)
 - Actix web (4.9)
 - Bincode (1.3)

## Train

The train (POST) endpoint is ```localhost:8080/train```. In the request body (JSON) there are 2 expected fields:
1. **text** - the text for training
2. **class** - the class that the text should be classified as

The response is either `Model successfully trained and data saved!` or an error message.

### Example

```json
{
    "text": "This is an example text.",
    "class": "example"
}
```


## Predict

The predict (POST) endpoint is ```localhost:8080/predict```. In the request body (JSON) there is 1 expected field:
1. **text** - the text that's class is predicted

The response is either the predicted class or an error message.

### Example

```json
{
    "text": "This is another example text"
}
```

## Saving and loading

As said before the model saves it self after every training.

The `model.bin` binary is the latest state of the model and this gets overwritten on each save.

```
any-directory/
├── NaiveBayes
├── model/
|   ├── model.bin
|   └── versions/
|       ├── model--<class>--<date>.bin
|       └── ...
```

When the model is initialized it searches for a `/model` folder in the same directory as the model is in. The binary that is loaded on the initialization is `/model/model.bin`.

If the model does not find `/model` folder it will create it.

In order to run a version an older version of the model, the version should be renamed to `model.bin` and put in the `/model` folder instead of the latest one.

Also the version files are not necessary and can be deleted at any moment.