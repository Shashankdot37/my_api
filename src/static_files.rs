use std::path::{Path,PathBuf};
use rocket::get;
use rocket::fs::NamedFile;

#[get("/")]
pub async fn index() -> Option<NamedFile>{
    NamedFile::open("public/index.html").await.ok()
}

#[get("/<file..>", rank = 5)]
pub async fn all(file: PathBuf) ->Option<NamedFile>{
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}