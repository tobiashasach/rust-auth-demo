use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use demo_auth::{
    models::users::{NewUser, User},
    response::Response,
};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::AppState;

pub async fn new_user(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>,
) -> Response<User> {
    use demo_auth::schema::users;

    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(_) => return Response::Err(StatusCode::INTERNAL_SERVER_ERROR, None),
    };

    let result = diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await;

    match result {
        Ok(user) => Response::Success(user),
        Err(_) => Response::Err(StatusCode::INTERNAL_SERVER_ERROR, None),
    }
}
