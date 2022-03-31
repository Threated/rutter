use std::path::Path;
use redis::AsyncCommands;
use rocket::fs::NamedFile;
use rocket_db_pools::Connection;

use crate::Db;


#[get("/<key>")]
pub async fn get(key: String, mut db: Connection<Db>) -> String {      
    format!("Got, {}!", db.get(key).await.unwrap_or("nothing matching the key".to_owned()))
}

#[get("/<key>/<value>")]
pub async fn set(key: String, value: String, mut db: Connection<Db>) -> &'static str {
    match db.set::<_, _, ()>(key, value).await {
        Ok(_) => "Sucess",
        Err(_) => "Faild"
    }
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./frontend/index.html")).await.ok()
}
