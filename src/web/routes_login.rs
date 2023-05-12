use crate::{web, Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookie: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("-> {:<12} - api_login", "HANDLER");

    // TODO: Implement real auth logic
    if (payload.username != "demo1" || payload.pwd != "welcome") {
        return Err(Error::LoginFail);
    }

    // FIXME: implement real cookie generation
    cookie.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create the succes body
    let body = Json(json!({
        "result": {
            "sucsess" : true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
