//find_match.rs

use axum::{extract::State, routing::get, Router};
use reqwest::Client;
use crate::AppState; 
use crate::routes::matchmaking_status::{get_matchmaking_status, MatchmakingState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}

async fn handler(State(s): State<AppState>) -> Result<String, String> {
    println!("Starting match search...");
    let search_state = get_matchmaking_status(&s.base_url, &s.token, &s.client).await?;

    find_match(&s.base_url, &s.token, &s.client, search_state).await
}

async fn find_match(base_url: &str, token: &str, client: &Client, search_state: MatchmakingState) -> Result<String, String> {
    let url = format!("{}/lol-lobby/v2/lobby/matchmaking/search", base_url);
    
    let request = if search_state == MatchmakingState::Searching {
        client.delete(url)
    } else {
        client.post(url)
    };
    
    let response = request
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

