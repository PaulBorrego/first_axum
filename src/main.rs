#![allow(unused)] // for beginning only

pub use self::error::{Error, Result};

// backend
use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
};

mod error;
mod web;

use serde::Deserialize;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().merge(routes_hello());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct SchedulerRequest {
    task: Option<String>,
}

// /scheuler?task=example
async fn handler_scheduler(Query(params): Query<SchedulerRequest>) -> impl IntoResponse {
    println!("->> scheduler_handler - {:?}", params);

    let task = params.task.as_deref().unwrap_or("No task provided");
    Html(format!(
        "<h1>Scheduler</h1><h2>{task}</h2><p>This is the scheduler page.</p>"
    ))
}

// /scheduler/Paul
async fn handler_scheduler_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> scheduler_handler - {:?}", name);

    Html(format!(
        "<h1>Scheduler</h1><h2>{name}</h2><p>This is the scheduler page.</p>"
    ))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/scheduler", get(handler_scheduler))
        .route("/scheduler/{name}", get(handler_scheduler_path))
}
