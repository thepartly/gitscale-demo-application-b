//! application-b: greets by name, shouting.

use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct HelloQuery {
    name: Option<String>,
}

/// What `GET /api/b/hello` answers; the SDK's `Hello` mirrors it.
#[derive(Serialize)]
struct Hello {
    message: String,
    /// Which service answered.
    service: &'static str,
}

fn message(name: &str) -> String {
    hello::greet(name, hello::Style::Shout)
}

async fn hello(Query(query): Query<HelloQuery>) -> Json<Hello> {
    let name = query.name.unwrap_or_else(|| "world".to_string());
    Json(Hello {
        message: message(&name),
        service: "application-b",
    })
}

async fn shutdown() {
    let term = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("a SIGTERM handler")
            .recv()
            .await;
    };
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = term => {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init("application-b", logging::Format::Json);
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let app = Router::new()
        .route("/api/b/hello", get(hello))
        .route("/healthz", get(|| async { "ok" }));
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    tracing::info!(port, "listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_shouting() {
        assert_eq!(message("Ada"), "HELLO, ADA!");
    }
}
