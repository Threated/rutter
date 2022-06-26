use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType};



fn client() -> Client {
    Client::tracked(rocket()).expect("valid rocket instance")
}

#[test]
fn index() {
    let client = client();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn auth() {
    let client = client();
    let user_data = r#"{
        "name": "Test",
        "password": "test"
    }"#;
    
    let response = client.post("/auth/login")
        .header(ContentType::JSON)    
        .body(user_data)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
    
    let response = client.post("/auth/register")
        .header(ContentType::JSON)
        .body(user_data)
        .dispatch();
    assert_eq!(response.status(), Status::Created);
    
    let response = client.post("/auth/register")
        .header(ContentType::JSON)    
        .body(user_data)
        .dispatch();
    assert_eq!(response.status(), Status::Conflict);
    
    let response = client.post("/auth/login")
        .header(ContentType::JSON)    
        .body(user_data)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.get("/public/u/Test")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Add assert
    
    let response = client.delete("/user")
        .header(ContentType::JSON)
        .cookie(response.cookies().get("session_id").unwrap().to_owned())    
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // TODO add logout test like ??? how does remove add private work
}
