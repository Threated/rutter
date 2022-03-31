mod public;
use crate::public::*;

#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, deadpool_redis};

#[derive(Database)]
#[database("redis")]
pub struct Db(deadpool_redis::Pool);


// fn connect_redis() -> redis::Client {
//     let client = redis::Client::open("redis://localhost/").unwrap();
//     // client.get_connection().unwrap()
//     client
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", rocket::routes![get, set, index])
}
