use axum::{
    body::Body,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{fs::read_to_string, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = app_route().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn app_route() -> Router {
    let app = Router::new()
        .route("/", get(root))
        .route("/about", get(about))
        .route("/styles/style.css", get(style_css));
    app
}

async fn root() -> impl IntoResponse {
    match read_to_string("frontend/index.html") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

async fn about() -> impl IntoResponse {
    match read_to_string("frontend/about.html") {
        Ok(content) => Html(content),
        Err(e) => Html(format!("Error : {}", e)),
    }
}

async fn style_css() -> impl IntoResponse {
    match read_to_string("src/frontend/styles/style.css") {
        Ok(content) => Response::builder()
            .header("Content-Type", "text/css")
            .body(Body::from(content))
            .unwrap(),
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("Error : {}", e)))
            .unwrap(),
    }
}
