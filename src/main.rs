use axum::serve;
use dotenvy::dotenv;
use std::env;
use tokio::signal;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT deve essere un numero valido");

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("Impossibile collegarsi alla porta");

    let app = routes::create_routes()
        .layer(TraceLayer::new_for_http());

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
