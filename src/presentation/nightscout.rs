use crate::{
    entities::{errors::AppResult, GlucoseMeasure},
    services::AppState,
};
use axum::{extract::State, Json};

pub async fn post_entries(
    State(state): State<AppState>,
    Json(data): Json<Vec<GlucoseMeasure>>,
) -> AppResult<Json<Vec<GlucoseMeasure>>> {
    let failed = state.repo.add_measures(data).await?;
    Ok(Json(failed))
}
