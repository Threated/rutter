use rocket::{http::Status, serde::json::json};

use crate::{auth::Authenticated, db::Redis, types::JsonRes};


#[post("/follow", format="json", data="<follow>")]
async fn follow(follow: &str, user: Authenticated, mut db: Redis) -> JsonRes {
    db.follow(&user.name, follow).await;
    (Status::Ok, json!({
        "message": format!("Followed {:?}", user.name)
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![follow]
}
