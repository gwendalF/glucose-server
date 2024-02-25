use crate::services::{auth_middleware, AppState};
use axum::{
    routing::{get, post},
    Router,
};

use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
mod hellowolrd;
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
        .route("/", get(hellowolrd::default))
        .nest("/api/v1", nigthscout_routes)
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state)
        .layer(LiveReloadLayer::new())
}
