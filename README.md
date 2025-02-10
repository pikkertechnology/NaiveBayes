# Naive Bayes

This is a Naive Bayes classifier model written in Rust. It is part of the [Study Material Sharing Website](https://openabi.ee), where it is used to classify different categories of uploaded materials.

## How It Works

The model provides two endpoints: **train** and **predict**. These endpoints allow the model to function seamlessly within a [***Docker container***](https://www.docker.com/). The endpoints are implemented using [Actix Web](https://actix.rs/).

The model saves its state to a `.bin` file in the `/model` folder. Each training session is saved as a separate version, making it easy to roll back to a previous model state.

## Technologies Used

- **Rust** (2021 Edition)
- **Actix Web** (4.9)
- **Bincode** (1.3)

## Endpoints

### Train Endpoint

**URL:** `localhost:8080/train`  
**Method:** POST  

The train endpoint expects a JSON request body with the following fields:

1. **text** - The training text.
2. **class** - The category to which the text should be classified.

**Response:**  
Returns either `Model successfully trained and data saved!` or an error message.

#### Example Request:
```json
{
    "text": "This is an example text.",
    "class": "example"
}
```

### Predict Endpoint

**URL:** `localhost:8080/predict`  
**Method:** POST  

The predict endpoint expects a JSON request body with the following field:

1. **text** - The text whose class is to be predicted.

**Response:**  
Returns the predicted class or an error message.

#### Example Request:
```json
{
    "text": "This is another example text."
}
```

## Saving and Loading

The model automatically saves its state after each training session. The primary save file is `model.bin`, located in the `/model` folder. This file represents the latest state of the model and is overwritten with each save.

### File Structure:
```plaintext
any-directory/
├── NaiveBayes
├── model/
|   ├── model.bin
|   └── versions/
|       ├── model--<class>--<date>.bin
|       └── ...
```

### Initialization:

When initialized, the model searches for a `/model` folder in the same directory as the executable. If the folder does not exist, it will be created automatically. The binary file `/model/model.bin` is loaded during initialization.

### Rolling Back:

To use an older version of the model, rename the desired version file to `model.bin` and place it in the `/model` folder, replacing the current file. Note that the version files are optional and can be deleted at any time.

## Training the Model

The recommended way to train the model is locally. Follow these steps:

1. Build the Naive Bayes binary using:
   ```bash
   cargo build --release
   ```
2. Run the binary from the `/target/release/` directory. This will start the server and open the endpoints for training.

### Post-Training:

After training, the `/model/model.bin` file is the only important artifact. You can delete everything else, including the compiled binary.

To deploy the trained model, place the `model.bin` file into a Docker container. This ensures the model is ready for usage without further configuration.
The file should be put in a `/model` folder alongside the `NaiveBayes` binary.