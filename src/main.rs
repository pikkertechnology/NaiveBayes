use actix_web::{post, web, HttpResponse, Responder, App, HttpServer};
use log::{debug, error, info};
use naive_bayes::NaiveBayesModel;
use serde::Deserialize;
use std::env;
use std::sync::Mutex;
mod naive_bayes;

struct AppState {
    model: Mutex<NaiveBayesModel>,
}

#[derive(Deserialize)]
struct TrainForm {
    text: String,
    class: String,
}

#[derive(Deserialize)]
struct PredictForm {
    text: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting the model.");
    let model = NaiveBayesModel::new();
    let app_state = web::Data::new(AppState {
        model: Mutex::new(model),
    });

    info!("Starting the HTTP server.");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(train)
            .service(predict)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/train")]
async fn train(form: web::Json<TrainForm>, data: web::Data<AppState>) -> impl Responder {
    info!("A train request recieved.");

    let class = &form.class;
    let text = &form.text;

    match data.model.lock() {
        Ok(mut model) => {
            debug!("Aquired lock on the model successfully.");

            let result = model.train(text.to_string(), class.to_string());
            HttpResponse::Ok().body(result.unwrap())
        }
        Err(e) => {
            error!("Failed to aquire lock on model: {e}");

            HttpResponse::InternalServerError()
                .body(format!("Failed to acquire lock on model: {}", e))
        }
    }
}

#[post("/predict")]
async fn predict(form: web::Json<PredictForm>, data: web::Data<AppState>) -> impl Responder {
    info!("A predict request recieved.");
    let text = &form.text;

    match data.model.lock() {
        Ok(model) => {
            debug!("Aquired lock on the model successfully.");

            let result = model.predict(text.to_string());
            HttpResponse::Ok().body(result.unwrap())
        }
        Err(e) => {
            error!("Failed to aquire lock on model: {e}");

            HttpResponse::InternalServerError()
                .body(format!("Failed to acquire lock on model: {}", e))
        }
    }
}
