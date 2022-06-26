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


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::Db::init())
        .mount("/", FileServer::from("static/"))
        .mount("/auth", auth::routes())
        .mount("/user", user::routes())
}
