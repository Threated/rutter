use rocket::{serde::json::Value, http::Status};


pub type JsonRes = (Status, Value);
