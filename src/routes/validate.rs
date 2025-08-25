use crate::{
    models::{ValidateRequest, ValidateResponse},
    routes::token::AppState,
};
use axum::{Json, extract::State};
use std::time::SystemTime;

pub async fn validate_token(
    State(state): State<AppState>,
    Json(req): Json<ValidateRequest>,
) -> Json<ValidateResponse> {
    if let Some(entry) = state.tokens.get(&req.token) {
        let now = SystemTime::now();
        if now <= entry.exp && entry.audience == req.self_name {
            return Json(ValidateResponse { active: true });
        }
    }
    Json(ValidateResponse { active: false })
}
