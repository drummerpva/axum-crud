use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_teste::{
    model::UsersDb,
    user_service::{create_user, delete_user, get_user, get_users, update_user},
};
use serde_json::{json, Value};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let users_db = UsersDb::default();
    let users_api = Router::new()
        .route("/", post(create_user).get(get_users))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(users_db);
    let api = Router::new()
        .nest("/users", users_api)
        .fallback(api_fallback);
    let app = Router::new().nest("/api", api);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404,
        "message": "Not Found",
    });

    return (StatusCode::NOT_FOUND, Json(body));
}
