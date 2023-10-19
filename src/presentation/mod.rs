use crate::services::{auth_middleware, AppState};
use axum::{
    routing::{get, post},
    Router,
};
mod nightscout;

pub fn get_router(state: AppState) -> Router {
    let nigthscout_routes = Router::new()
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .route("/entries", post(nightscout::post_entries))
        .route("/entries", get(nightscout::get));
    Router::new()
        .nest("/api/v1", nigthscout_routes)
        .with_state(state)
}
