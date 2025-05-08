#![allow(unused)] // for beginning only

pub use self::error::{Error, Result};

// backend
use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse,Response},
    routing::get,
    middleware,
};

mod error;
mod web;
mod model;

use crate::model::ModelController;

use serde::Deserialize;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;
    let routes_api = web::routes_tickets::routes(mc.clone()).route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // build our application with a single route
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Debug, Deserialize)]
struct ScheduleRequest {
    task: Option<String>,
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> main_response_mapper");
    
    res
}

// /scheuler?task=example
async fn handler_schedule(Query(params): Query<ScheduleRequest>) -> impl IntoResponse {
    println!("->> schedule_handler - {:?}", params);

    let task = params.task.as_deref().unwrap_or("No task provided");
    Html(format!(
        "<h1>Schedule</h1><h2>{task}</h2><p>This is the schedule page.</p>"
    ))
}

// /schedule/Paul
async fn handler_schedule_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> schedule_handler - {:?}", name);

    Html(format!(
        "<h1>Schedule</h1><h2>{name}</h2><p>This is the schedule page.</p>"
    ))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/schedule", get(handler_schedule))
        .route("/schedule/{name}", get(handler_schedule_path))
}
