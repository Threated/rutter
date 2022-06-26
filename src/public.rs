use std::path::Path;
use rocket::{fs::NamedFile, Route};

use crate::db::Redis;



#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./frontend/index.html")).await.ok()
}

#[get("/u/<user>")]
async fn get_user(user: &str, mut db: Redis) {
    db.get_user(user).await;
}

pub fn routes() -> Vec<Route> {
    routes!(index, get_user)
}
