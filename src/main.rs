#[cfg(test)] mod tests;
mod public;
mod db;
mod auth;
mod responders;
mod types;
mod user;
use rocket::fs::FileServer;
use rocket_db_pools::Database;
#[macro_use] extern crate rocket;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::Db::init())
        .attach(CORS)
        .mount("/", FileServer::from("static/"))
        .mount("/auth", auth::routes())
        .mount("/user", user::routes())
}
