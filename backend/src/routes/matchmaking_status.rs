//matchmaking_status.rs

use axum::{extract::State, routing::get, Router};
use reqwest::Client;
use crate::AppState;
use serde::{Deserialize, Serialize};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}

#[derive(Deserialize)]
struct SearchStateResponse {
    #[serde(rename = "searchState")]
    search_state: MatchmakingState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchmakingState {
    Invalid,
    Searching,
    Found,
    Error,
    #[serde(other)]
    Unknown,
}

pub async fn get_matchmaking_status(base_url: &str, token: &str, client: &Client) -> Result<MatchmakingState, String> {
    let url = format!("{}/lol-lobby/v2/lobby/matchmaking/search-state", base_url);

    let response = client
        .get(url)
        .basic_auth("riot", Some(token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    let body = response.text().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        return Err(format!("LCU {}: {}", status.as_u16(), body));
    }

    let parsed: SearchStateResponse = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse search-state response: {}", e))?;

    Ok(parsed.search_state)
}

async fn handler(State(s): State<AppState>) -> Result<String, String> {
    let state = get_matchmaking_status(&s.base_url, &s.token, &s.client).await?;
    Ok(serde_json::json!({ "status": state }).to_string())
}