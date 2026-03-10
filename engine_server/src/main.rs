use axum::{routing::get, Router, Json};
use engine::HtmlParser;
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct ParseResponse {
    message: String,
    engine_status: String,
}

#[tokio::main]
async fn main() {
    // Railway provides a PORT environment variable
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

    let app = Router::new().route("/", get(handler));

    println!("Engine Server running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<ParseResponse> {
    // Use your engine to parse a dummy string for the demo
    let mut parser = HtmlParser::new("<html><body></body></html>".to_string());
    let _dom = parser.parse_nodes();

    Json(ParseResponse {
        message: "Your custom Rust Engine is live!".to_string(),
        engine_status: "DOM successfully initialized".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler() {
        let response = handler().await;
        let json_value = serde_json::to_value(response.0).unwrap();
        
        assert_eq!(json_value["message"], "Your custom Rust Engine is live!");
        assert_eq!(json_value["engine_status"], "DOM successfully initialized");
    }
}