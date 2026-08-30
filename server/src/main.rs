mod routes;

use axum::Router;
use axum::http::Method;
use dotenv::dotenv;
use migration::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use std::env;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: String = env::var("PORT").unwrap_or("9000".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let (app, state) = create_app(&database_url).await;

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://localhost:{port}");

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            // wait for Ctrl+C signal
            let _ = signal::ctrl_c().await;

            // perform cleanup tasks here
            println!("Received shutdown signal, shutting down gracefully...");
        })
        .await
        .unwrap();
}

async fn create_app(database_url: &str) -> (Router, AppState) {
    // ensure database_url starts with "postgres://"
    if !database_url.starts_with("postgres://") {
        panic!("Unexpected DATABASE_URL: {}", database_url);
    }

    // create PostgreSQL DB connection
    let db = match Database::connect(database_url).await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("❌ Database connection failed: {err}");
            eprintln!("Hint:");
            eprintln!("  1. Ensure services are running: cargo make services:up");
            eprintln!("  2. Check DATABASE_URL");
            eprintln!("  3. Ensure DB is healthy: docker compose -f compose.services.dev.yml ps");
            std::process::exit(1);
        }
    };

    // run database migrations to ensure the database is up to date
    println!("[startup] Running migrations...");
    Migrator::up(&db, None).await.expect("Migration failed");
    println!("[startup] Migrations completed successfully");

    let state = AppState { db: db };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .merge(routes::router())
        .layer(cors)
        .with_state(state.clone());
    return (app, state);
}
