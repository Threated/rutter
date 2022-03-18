#[macro_use] extern crate rocket;
extern crate redis;
use redis::{Commands, Connection, RedisError};


fn get_connection() -> Result<Connection, RedisError> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    client.get_connection()
}

#[get("/<key>")]
fn get(key: String) -> String {

    let mut con = get_connection().unwrap();
    
    format!("Got, {}!", con.get::<_, String>(key).unwrap_or("Nothing".to_owned()))
}

#[get("/<key>/<value>")]
fn set(key: String, value: String) -> &'static str {
    let mut con = get_connection().unwrap();
    let _ : () = con.set(key, value).unwrap();
    "success"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get, set])
}
