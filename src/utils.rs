use actix_multipart::form::tempfile::TempFile;
use extractous::Extractor;
use std::i32::MAX;

pub fn extract_file_content(file: &TempFile) -> Result<String, String> {
    let file_type = file
        .content_type
        .as_ref()
        .ok_or_else(|| "Missing content type".to_string())?;

    let file_path = file
        .file
        .path()
        .to_str()
        .ok_or_else(|| "Failed to convert path to string".to_string())?;

    let extractor = Extractor::new().set_extract_string_max_length(MAX);

    match extractor.extract_file_to_string(file_path) {
        Ok((text, _)) => Ok(text),
        Err(err) => Err(format!(
            "Failed to extract file: {:?}, file type: {}",
            err, file_type
        )),
    }
}
