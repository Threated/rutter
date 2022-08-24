#[cfg(test)] mod tests;
mod db;
mod auth;
mod responders;
mod types;
mod user;

use rocket::{fs::FileServer, fairing::AdHoc};
use rocket_cors::AllowedOrigins;
use rocket_db_pools::Database;
#[macro_use] extern crate rocket;


// TODO: Just use RedisLabs/redisgraph docker image
// Impl actual search
// Just sort matches by follows

#[launch]
fn rocket() -> _ {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&["http://127.0.0.1:8000", "http://localhost:3000"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap();

    rocket::build()
        .attach(db::Db::init())
        .attach(AdHoc::try_on_ignite("DB Migrations", db::db_migrations))
        .attach(cors)
        .mount("/", FileServer::from("static/"))
        .mount("/auth", auth::routes())
        .mount("/user", user::routes())
}
