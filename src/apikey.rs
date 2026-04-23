use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn require_api_key(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let expected = env::var("API_KEY").expect("API_KEY env var not set");

    let provided = req
        .headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if provided != expected {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}