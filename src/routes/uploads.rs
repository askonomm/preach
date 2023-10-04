use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;

#[get("/<file..>")]
pub async fn uploads(file: PathBuf) -> Option<NamedFile> {
    let file_path = Path::new(&file);
    let result = NamedFile::open(file_path).await;

    result.ok()
}
