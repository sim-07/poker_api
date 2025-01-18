use axum::serve;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::trace::TraceLayer;

mod connect_db;
mod handlers;
mod routes;
mod session;
mod ws;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT non valido");

    let db_pool = connect_db::connect_db().await;
    println!("Database pool: {:?}", db_pool);

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Impossibile collegarsi alla porta");

    // Cookie
    let cookie_key = session::get_cookie_key();
    let shared_state = Arc::new(cookie_key);

    // Routes
    let app = routes::create_routes(db_pool, shared_state).layer(TraceLayer::new_for_http());

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
