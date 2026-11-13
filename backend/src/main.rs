use axum::Router;
use reqwest::Client;

mod get_token;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub base_url: String,
    pub token: String,
    pub client: Client,
}

#[tokio::main]
async fn main() {
    let Some((port, token)) = get_token::get_lcu_token() else {
        eprintln!("LCU não encontrado (rodou como Admin?)");
        std::process::exit(1);
    };

    println!("LCU encontrado na porta {port} e token {token}");

    let state = AppState {
        base_url: format!("https://127.0.0.1:{port}"),
        token,
        client: Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
    };

    let app = Router::new()
        .nest("/accept_match",          routes::accept_match::router())
        .nest("/enter_aram_mayhem",     routes::enter_aram_loby::router())
        .nest("/find_match",            routes::find_match::router())
        .nest("/matchmaking_status",    routes::matchmaking_status::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}