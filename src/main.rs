use anyhow::{Ok, Result};
use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    response::Html,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use db::song_repo;
use scanner::Scanner;
use sea_orm::DatabaseConnection;
use std::env;
use std::net::SocketAddr;
use tower::Service;
use tower_http::{services::ServeFile, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod scanner;
#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");

    // Setup tracing logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Connecting SQLite

    let db = db::get_connection().await?;
    db::migrate_up(&db).await?;

    let mut scan = scanner::Scanner::new().unwrap();
    scan.start_scan();
    println!("{:?}", scan.get_status());
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/stream/:id", get(stream_handler))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db))
        .layer(Extension(scan));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

struct Params {
    song_id: String,
}

async fn stream_handler(
    Path(song_id): Path<String>,
    Extension(ref db): Extension<DatabaseConnection>,
    req: Request<Body>,
) -> impl IntoResponse {
    let song = song_repo::get_song(db, song_id)
        .await
        .expect("Unknown Song");
    if song.is_none() {
        return (StatusCode::NOT_FOUND, "Song not found").into_response();
    } else {
        let path = song.unwrap().path;

        return ServeFile::new(path)
            .call(req)
            .await
            .unwrap()
            .into_response();
    }
}

async fn handler(
    Extension(ref scanner): Extension<Scanner>,
    Extension(ref db): Extension<DatabaseConnection>,
) -> Html<&'static str> {
    /*     let id = Uuid::new_v4();
    entity::artists::ActiveModel {
        id: Set(id.to_string().to_owned()),
        name: Set("test".to_owned()),
        image: NotSet,
        bio: NotSet,
        created_at: Set(Utc::now().naive_local()),
        updated_at: Set(Utc::now().naive_local()),
    }
    .insert(pool)
    .await
    .expect("Failed to insert"); */
    println!("{:?}", scanner.get_status());
    Html("<h1>{Hello, World}!</h1>")
}
