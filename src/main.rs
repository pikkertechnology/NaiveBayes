use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::{App, HttpServer};
use naive_bayes::NaiveBayesModel;
use std::sync::Mutex;
mod naive_bayes;
mod utils;

struct AppState {
    model: Mutex<NaiveBayesModel>,
}

#[derive(Debug, MultipartForm)]
struct TrainForm {
    file: TempFile,
    class: Text<String>,
}

#[derive(Debug, MultipartForm)]
struct PredictForm {
    file: TempFile,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let model = NaiveBayesModel::new();
    let app_state = web::Data::new(AppState {
        model: Mutex::new(model),
    });

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
async fn train(
    MultipartForm(form): MultipartForm<TrainForm>,
    data: web::Data<AppState>,
) -> impl Responder {
    let class = form.class.to_string();
    let file = form.file;

    match data.model.lock() {
        Ok(mut model) => {
            let result = model.train(file, class);
            HttpResponse::Ok().body(result.unwrap())
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to acquire lock on model: {}", e)),
    }
}

#[post("/predict")]
async fn predict(MultipartForm(form): MultipartForm<PredictForm>, data: web::Data<AppState>,
) -> impl Responder {
    let file = form.file;

    match data.model.lock() {
        Ok(model) => {
            let result = model.predict(file);
            HttpResponse::Ok().body(result.unwrap())
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to acquire lock on model: {}", e))
        
    }

}
