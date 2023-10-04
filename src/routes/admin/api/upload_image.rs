use std::env;

use dotenvy::dotenv;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{form::Form, http::CookieJar};
use serde::Deserialize;

use crate::utils::data::is_authenticated;

#[derive(FromForm)]
pub struct UploadImage<'r> {
    file: TempFile<'r>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadImageResponse {
    status: String,
    file_name: Option<String>,
    file_url: Option<String>,
}

#[post("/admin/api/upload-image", data = "<upload>")]
pub async fn upload_image(
    cookies: &CookieJar<'_>,
    mut upload: Form<UploadImage<'_>>,
) -> Json<UploadImageResponse> {
    if !is_authenticated(cookies) {
        return Json(UploadImageResponse {
            status: "error".to_string(),
            file_name: None,
            file_url: None,
        });
    }

    dotenv().ok();

    let upload_path = env::var("UPLOAD_PATH").expect("UPLOAD_PATH must be set");
    let upload_url = env::var("UPLOAD_URL").expect("UPLOAD_URL must be set");
    let file = &upload.file;
    let ext = file.content_type().unwrap().extension().unwrap();
    let name = format!(
        "{}_{}.{}",
        file.name().unwrap(),
        chrono::Utc::now().timestamp(),
        ext
    );

    let result = upload
        .file
        .persist_to(format!("{}/{}", upload_path, name))
        .await;

    match result {
        Ok(_) => {
            return Json(UploadImageResponse {
                status: "success".to_string(),
                file_name: Some(name.clone()),
                file_url: Some(format!("{}/{}", upload_url, name)),
            });
        }
        Err(error) => {
            println!("{:?}", error);

            return Json(UploadImageResponse {
                status: "error".to_string(),
                file_name: None,
                file_url: None,
            });
        }
    }
}
