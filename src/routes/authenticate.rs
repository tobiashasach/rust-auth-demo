use std::sync::{Arc, LazyLock};

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use demo_auth::{jwt::jwt_sign, models::users::User, password::password_verify, schema};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use zeroize::Zeroizing;

use crate::AppState;

static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| {
    demo_auth::password::password_hash(Zeroizing::new(String::from(
        "dummy_password_for_timing_protection",
    )))
    .unwrap()
});

#[derive(Deserialize, Debug)]
pub struct AuthorizePayload {
    email: String,
    password: String,
}

pub async fn authenticate(
    State(state): State<Arc<AppState>>,
    Json(authorize_payload): Json<AuthorizePayload>,
) -> Response {
    use schema::users;

    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let user_option: Result<User, diesel::result::Error> = users::table
        .filter(users::email.eq(authorize_payload.email))
        .first(&mut conn)
        .await;

    let (target_hash, user_exists) = match &user_option {
        Ok(user) => (user.password.as_str(), true),
        Err(_) => (DUMMY_HASH.as_str(), false),
    };

    let password_valid =
        password_verify(Zeroizing::new(authorize_payload.password), target_hash).is_ok();

    if !user_exists || !password_valid {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let user = user_option.unwrap();

    let (jwt_token, epires) = match jwt_sign(&user.id.to_string()) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let mut headers = HeaderMap::new();
    match HeaderValue::from_str(&format!(
        "jwt={}; HttpOnly; Secure; SameSite=Lax; Path=/; Expires={}",
        jwt_token,
        epires.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
    )) {
        Ok(header_val) => headers.insert(SET_COOKIE, header_val),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    headers.into_response()
}
