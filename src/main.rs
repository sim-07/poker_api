use axum::serve;
use cookie::Key;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::trace::TraceLayer;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

mod connect_db;
mod handlers;
mod routes;
mod session;
mod ws;
mod redis_store;

#[derive(Clone)]
pub struct SharedState {
    pub cookie_key: Key,
    pub redis_pool: Arc<Pool<RedisConnectionManager>>,
    pub db_pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT non valido");

    // Connessione al db
    let db_pool = connect_db::connect_db().await;
    println!("Database pool: {:?}", db_pool);

    // Connessione Redis
    let redis_manager = RedisConnectionManager::new("redis://127.0.0.1").unwrap();
    let redis_pool = Arc::new(Pool::builder().build(redis_manager).await.unwrap());

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Impossibile collegarsi alla porta");

    // Cookie
    let cookie_key = session::get_cookie_key();

    // Stato condiviso
    let shared_state = Arc::new(SharedState {
        cookie_key,
        redis_pool,
        db_pool: Arc::new(db_pool)
    });

    // Routes
    let app = routes::create_routes(shared_state).layer(TraceLayer::new_for_http());

    println!("Server in ascolto su http://localhost:{port}");
    serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Errore durante l'avvio del server");
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Errore durante l'ascolto del segnale di interruzione");
    println!("Server in arresto...");
}
