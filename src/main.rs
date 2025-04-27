use axum::{Router, response::Json, routing::get};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Game {
    id: String,
    name: String,
    cover_url: Option<String>,
}

async fn get_games() -> Json<Vec<Game>> {
    let games = vec![
        Game {
            id: "1".to_string(),
            name: "Game 1".to_string(),
            cover_url: Some("https://via.placeholder.com/150".to_string()),
        },
        Game {
            id: "2".to_string(),
            name: "Game 2".to_string(),
            cover_url: Some("https://via.placeholder.com/150".to_string()),
        },
    ];

    Json(games)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/games", get(get_games));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    println!("Server running at http://localhost:5000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
