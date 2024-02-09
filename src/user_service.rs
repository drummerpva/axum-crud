use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{User, UsersDb};

#[derive(Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub username: String,
}

pub async fn create_user(
    State(users_db): State<UsersDb>,
    Json(input): Json<CreateUserInput>,
) -> impl IntoResponse {
    let user = User {
        id: Uuid::new_v4(),
        name: input.name,
        username: input.username,
    };
    users_db.write().unwrap().insert(user.id, user.clone());

    return (StatusCode::CREATED, Json(user));
}

#[derive(Serialize)]
pub struct GetUserOutput {
    pub name: String,
    pub username: String,
}

pub async fn get_user(
    Path(id): Path<Uuid>,
    State(users_db): State<UsersDb>,
) -> Result<impl IntoResponse, StatusCode> {
    let users = users_db.read().unwrap();
    if let Some(user) = users.get(&id).cloned() {
        let output = GetUserOutput {
            name: user.name,
            username: user.username,
        };
        return Ok((StatusCode::OK, Json(output)));
    }

    return Err(StatusCode::NOT_FOUND);
}
