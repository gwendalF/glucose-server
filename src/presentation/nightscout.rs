use crate::{
    entities::{errors::AppResult, GetDataQuery, GlucoseMeasure},
    services::AppState,
};
use axum::{
    extract::{Query, State},
    Json,
};
// use axum_macros::debug_handler;

pub async fn post_entries(
    State(state): State<AppState>,
    Json(data): Json<Vec<GlucoseMeasure>>,
) -> AppResult<Json<Vec<GlucoseMeasure>>> {
    let failed = state.repo.add_measures(1, data).await?;
    Ok(Json(failed))
}

// #[debug_handler]
pub async fn get(
    State(state): State<AppState>,
    Query(date): Query<GetDataQuery>,
) -> AppResult<Json<Vec<GlucoseMeasure>>> {
    let data = state.repo.get_measures(date, 1).await?;
    Ok(Json(data))
}
