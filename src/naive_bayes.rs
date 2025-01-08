use std::fs;
use actix_multipart::form::{tempfile::TempFile, MultipartForm, text::Text};
use actix_web::{post, HttpResponse, Responder};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    file: TempFile,
    class: Text<String>,
}

#[post("/train")]
async fn train(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let file_content = fs::read_to_string(&form.file.file.path())
        .unwrap_or_else(|_| "Failed to read file".to_string());
    let file_type = form.file.content_type.as_ref().unwrap();

    format!(
        "Uploaded file with class: {}, size: {} bytes, type: {}, with content: {}",
        form.class.as_str(),
        form.file.size,
        file_type,
        file_content
    )
}

#[post("/predict")]
async fn predict(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
