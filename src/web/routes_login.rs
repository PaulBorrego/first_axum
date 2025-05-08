use crate::{Error, Result};
use serde::Deserialize;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};

use tower_cookies::{Cookies, Cookie};
use crate::web;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> api_login");

    // ToDo: Implement users
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // FIXME: create actual auth-token generator
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

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