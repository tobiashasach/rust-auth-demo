use std::{error::Error, sync::Arc};

use axum::{
    Json, Router,
    routing::{get, post},
};
use demo_auth::config::CONFIG;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};

use crate::routes::{authorize::authorize, users::new_user};

mod routes;

struct AppState {
    pool: Pool<AsyncPgConnection>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let socket_addr = format!("{}:{}", CONFIG.host(), CONFIG.port());

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(CONFIG.database_url());
    let pool = Pool::builder().build(config).await?;

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/user", post(new_user))
        .route("/authorize", post(authorize))
        .with_state(state);

    println!("Listening on {}", socket_addr);

    let listener = tokio::net::TcpListener::bind(socket_addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn healthcheck() -> Json<String> {
    Json(format!("Hello from {}", String::from(CONFIG.name())))
}
