//accept_match.rs

use axum::{extract::State, routing::get, Router};
use reqwest::Client;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}

async fn handler(State(s): State<AppState>) -> Result<String, String> {
    println!("Accepting match...");
    accept_match(&s.base_url, &s.token, &s.client).await
}

async fn accept_match(base_url: &str, token: &str, client: &Client) -> Result<String, String> {
    let url = format!("{}/lol-matchmaking/v1/ready-check/accept", base_url);

    let response = client
        .post(url)
        .basic_auth("riot", Some(token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    let body = response.text().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        return Err(format!("LCU {}: {}", status.as_u16(), body));
    }

    Ok(body)
}