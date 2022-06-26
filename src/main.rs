#[cfg(test)] mod tests;
mod public;
mod db;
mod auth;
mod responders;
mod types;
mod user;
use rocket_db_pools::Database;
#[macro_use] extern crate rocket;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::Db::init())
        .mount("/", public::routes())
        .mount("/auth", auth::routes())
        .mount("/user", user::routes())
}
