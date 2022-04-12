use std::path::Path;
use rocket::{fs::NamedFile, Route};



#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./frontend/index.html")).await.ok()
}

pub fn routes() -> Vec<Route> {
    routes!(index)
}
