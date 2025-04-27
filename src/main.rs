use axum::{Router, response::Json, routing::get};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::types::Uuid;
use std::net::SocketAddr;
use tokio;

#[derive(Serialize, Deserialize)]
struct Game {
    id: Uuid,
    name: String,
    cover_url: Option<String>,
}

async fn get_games(db_pool: PgPool) -> Json<Vec<Game>> {
    let games = sqlx::query!(
        r#"
        SELECT id, name, cover_url
        FROM game
        "#,
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    let games: Vec<Game> = games
        .into_iter()
        .map(|game| Game {
            id: game.id,
            name: game.name,
            cover_url: game.cover_url,
        })
        .collect();

    Json(games)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    let app = Router::new().route("/games", get(move || get_games(db_pool.clone())));

    let addr = "127.0.0.1:5000".parse::<SocketAddr>().unwrap();

    println!("Server running at http://localhost:5000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
