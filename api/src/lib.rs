use axum::http::Method;
use axum::{extract::State, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::*;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

struct AppState {
    todos: Vec<Todo>,
}

fn router() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let shared_state = Arc::new(AppState {
        todos: vec![Todo {
            id: "1".to_string(),
            title: "Learn Cloudflare Stack".to_string(),
            completed: false,
        }],
    });

    Router::new()
        .route("/", get(root))
        .route("/todos", get(todos))
        .with_state(shared_state)
        .layer(cors)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

async fn root() -> &'static str {
    "Hello Axum! :)"
}

async fn todos(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({ "todos": state.todos }))
}
