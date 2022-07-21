use rocket::{serde::json::{Json, serde_json::json, Value}, http::Status, response::Responder};

#[derive(Responder)]
pub struct JsonRes<T = Value>(pub (Status, Json<T>));

impl <T>From<(Status, Json<T>)> for JsonRes<T> {
    fn from(data: (Status, Json<T>)) -> Self {
        JsonRes(data)
    }
}

impl From<(Status, &str)> for JsonRes {
    fn from((status, msg): (Status, &str)) -> Self {
        JsonRes((status, Json(json!({
            "message": msg
        }))))
    }
}

impl From<(Status, Value)> for JsonRes {
    fn from((status, val): (Status, Value)) -> Self {
        JsonRes((status, Json(val)))
    }
}

impl From<&str> for JsonRes {
    fn from(msg: &str) -> Self {
        JsonRes((Status::Ok, Json(json!({
            "message": msg
        }))))
    }
}

impl From<String> for JsonRes {
    fn from(msg: String) -> Self {
        JsonRes((Status::Ok, Json(json!({
            "message": msg
        }))))
    }
}

