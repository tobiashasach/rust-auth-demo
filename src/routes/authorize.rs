use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use demo_auth::{models::users::User, response::Response, schema};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct AuthorizePayload {
    email: String,
    password: String,
}

pub async fn authorize(
    State(state): State<Arc<AppState>>,
    Json(authorize_payload): Json<AuthorizePayload>,
) -> Response<String> {
    use schema::users;

    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => return Response::Err(StatusCode::INTERNAL_SERVER_ERROR, None),
    };

    let user: Result<User, diesel::result::Error> = users::table
        .filter(users::email.eq(authorize_payload.email))
        .first(&mut conn)
        .await;

    unimplemented!("WIP")
}
