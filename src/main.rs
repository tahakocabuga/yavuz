use std::net::{Ipv4Addr, SocketAddr};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde_json::{Value, json};
use axum::{routing, Router, response::Json, extract::Extension,};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

use anyhow::{Context, Ok};
use sqlx::{Pool, Sqlite, sqlite::{SqlitePoolOptions, SqliteConnectOptions, SqliteJournalMode}, ConnectOptions};
use sqlx::Connection;
use std::str::FromStr;
use std::env;


mod controllers;
mod models;

#[macro_use]
extern crate lazy_static;

// Eğer test modunda ise databaseyi test databasesi ile değiştir.
lazy_static! {    
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap_or_else(|_| if cfg!(test) {
        "sqlite:testmarks.db"
     } else {
        "sqlite:marks.db"
    }.to_string());
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await?;
    Ok(())
}


async fn run() -> anyhow::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            controllers::mark::all_marks,
            controllers::mark::new_mark,
            controllers::mark::mark,
            controllers::mark::delete_mark,
        ),
        components(
            schemas(models::mark::Mark, models::mark::NewMark, models::mark::UpdateMark)
        ),
        tags(
            (name = "mark", description = "UAV Mark management API")
        )
    )]
    struct ApiDoc;


    init_tracing();

    let pool = prepare_database().await?;

    

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))

        .route("/", routing::get(test))
        .route("/marks", routing::get(controllers::mark::all_marks))
        .route("/marks/yeni", routing::post(controllers::mark::new_mark))
        .route("/marks/:id", routing::get(controllers::mark::mark))
        .route("/marks/:id", routing::delete(controllers::mark::delete_mark))

        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;
    println!("[RUST] API on: {}", address);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

fn init_tracing(){
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=debug,axum_crud_api=debug")
                .unwrap_or_else(|_| if cfg!(test) {
                    "tower_http=error"
                } else {
                    "axum_crud_api=debug,tower_http=debug"
                }.into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}



async fn test() -> Json<Value> {
    Json(json!({ "success": true, "message": "API çalışır durumda" }))
}

async fn prepare_database() -> anyhow::Result<Pool<Sqlite>> {
    
    // eğer database yok ise oluştur 
    let conn = SqliteConnectOptions::from_str(&DATABASE_URL)?
    .journal_mode(SqliteJournalMode::Wal).create_if_missing(true)
    .connect().await?;
    conn.close();

    // connection pool hazırla
    let pool = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&DATABASE_URL)
        .await
        .context("could not connect to database_url")?;

    // eğer oluşturulmamışsa schemayı oluştur
    sqlx::migrate!().run(&pool).await?;

    

    Ok(pool)
}