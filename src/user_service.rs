use axum::{
    extract::{Path, Query, State},
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
#[derive(Deserialize)]
pub struct UpdateUserInput {
    pub name: Option<String>,
    pub username: Option<String>,
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    State(users_db): State<UsersDb>,
    Json(input): Json<UpdateUserInput>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut user = users_db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;
    /* let updated_user = User {
        id: user.id,
        name: input.name.or(Some(user.name)).unwrap(),
        username: input.username.or(Some(user.username)).unwrap(),
    }; */
    if let Some(name) = input.name {
        user.name = name;
    }
    if let Some(username) = input.username {
        user.username = username;
    }
    users_db.write().unwrap().insert(id, user.clone());

    return Ok((StatusCode::CREATED, Json(user)));
}

pub async fn delete_user(Path(id): Path<Uuid>, State(users_db): State<UsersDb>) -> StatusCode {
    if users_db.write().unwrap().remove(&id).is_some() {
        return StatusCode::NO_CONTENT;
    }
    return StatusCode::NOT_FOUND;
}

#[derive(Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn get_users(
    pagination: Option<Query<Pagination>>,
    State(users_db): State<UsersDb>,
) -> impl IntoResponse {
    let users = users_db.read().unwrap();
    let pagination = pagination.unwrap_or_default();
    let users: Vec<User> = users
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect();
    return (StatusCode::OK, Json(users));
}
