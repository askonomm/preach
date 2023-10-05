use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    let file_path = Path::new(&file);
    let result = NamedFile::open(file_path).await;

    result.ok()
}
