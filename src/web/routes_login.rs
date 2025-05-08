use crate::{Error, Result};
use serde::Deserialize;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> api_login");

    // ToDo: Implement users
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO Set Cookies

    // Create the Success Body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug,Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}