use anyhow::Result;
use axum::{
    http::Method,
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use signup::{create_handler, login_handler, profile_handler, verify_customer, Db};
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

const ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let db = Db::try_new().await?;

    let listener = TcpListener::bind(ADDR).await?;

    info!("Server listening on {}", ADDR);

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    let app = Router::new()
        .route("/profile", get(profile_handler))
        .layer(from_fn_with_state(db.clone(), verify_customer))
        .route("/login", post(login_handler))
        .route("/create", post(create_handler))
        .layer(cors)
        .with_state(db);

    axum::serve(listener, app).await?;

    Ok(())
}
