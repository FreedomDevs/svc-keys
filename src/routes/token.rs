use crate::{
    models::{IssueRequest, IssueResponse, TokenEntry},
    permissions::{Permissions, is_allowed},
    store::TokenStore,
};
use axum::{Json, extract::State};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::RngCore;
use std::time::{Duration, SystemTime};

#[derive(Clone)]
pub struct AppState {
    pub tokens: TokenStore,
    pub permissions: Permissions,
}

pub async fn issue_token(
    State(state): State<AppState>,
    Json(req): Json<IssueRequest>,
) -> Json<IssueResponse> {
    if !is_allowed(&state.permissions, &req.issuer, &req.audience) {
        return Json(IssueResponse {
            token: "".to_string(),
            exp: 0,
        });
    }

    let token = generate_token();
    let exp_time = SystemTime::now() + Duration::from_secs(req.ttl_sec);

    let entry = TokenEntry {
        issuer: req.issuer,
        audience: req.audience,
        exp: exp_time,
    };
    state.tokens.insert(token.clone(), entry);

    Json(IssueResponse {
        token,
        exp: exp_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}

fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
}

