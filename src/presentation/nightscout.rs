use crate::{entities::GlucoseMeasure, services::AppState};
use axum::{extract::State, http::StatusCode, Json};

pub async fn post_entries(
    State(state): State<AppState>,
    Json(data): Json<Vec<GlucoseMeasure>>,
) -> (StatusCode, Json<Vec<GlucoseMeasure>>) {
    let failed = state.repo.add_measures(data).await.unwrap();
    (StatusCode::OK, Json(failed))
}
