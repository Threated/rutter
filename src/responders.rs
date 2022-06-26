use rocket::{serde::json::{Value, serde_json::json}, http::Status, response::Responder};

#[derive(Responder)]
pub struct JsonRes((Status, Value));

impl From<(Status, Value)> for JsonRes {
    fn from(data: (Status, Value)) -> Self {
        JsonRes(data)
    }
}

impl From<(Status, &str)> for JsonRes {
    fn from((status, msg): (Status, &str)) -> Self {
        JsonRes((status, json!({
            "message": msg
        })))
    }
}

impl From<&str> for JsonRes {
    fn from(msg: &str) -> Self {
        JsonRes((Status::Ok, json!({
            "message": msg
        })))
    }
}

impl From<String> for JsonRes {
    fn from(msg: String) -> Self {
        JsonRes((Status::Ok, json!({
            "message": msg
        })))
    }
}

