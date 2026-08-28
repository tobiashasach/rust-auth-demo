use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use demo_auth::{
    models::users::{NewUser, User},
    password::password_hash,
};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use zeroize::Zeroizing;

use crate::AppState;

#[derive(Deserialize)]
pub struct NewUserPayload {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

pub async fn new_user(
    State(state): State<Arc<AppState>>,
    Json(new_user_payload): Json<NewUserPayload>,
) -> Response {
    use demo_auth::schema::users;

    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let password_hash = match password_hash(Zeroizing::new(new_user_payload.password)) {
        Ok(h) => h,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let new_user = NewUser {
        email: new_user_payload.email,
        firstname: new_user_payload.firstname,
        lastname: new_user_payload.lastname,
        password: password_hash,
        created_at: None,
        updated_at: None,
    };

    let result: Result<User, diesel::result::Error> = diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await;

    match result {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}
