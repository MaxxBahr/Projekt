#![allow(unused)] //No warnings for unused variables

use std::fs;
use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router, ServiceExt};
use axum::routing::get;
use axum::*;
use axum::http::{Response, StatusCode};

#[tokio::main]
pub(crate) async fn listen() {
    //set all routes
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/test", get(|| async{Html("test <strong>test</strong>")}));

    //create a listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    //tell axum what the listener is and which routes we have
    axum::serve(listener, app).await.unwrap();
}
//Traits are like interfaces, so we can assume that "IntoResponse" has implemented
//a into_response() method.
//The impl lets us return everything that implements the "IntoResponse" trait
async fn serve_index() -> impl IntoResponse {
    // Dateiinhalt lesen
    match fs::read_to_string("../../frontend/index.html") {
        Ok(contents) => Html(contents).into_response(),
        Err(_) => (
            axum::http::StatusCode::NOT_FOUND,
            "Index file not found",
        )
            .into_response(),
    }
}