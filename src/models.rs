use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenEntry {
    pub issuer: String,
    pub audience: String,
    pub exp: SystemTime,
}

#[derive(Deserialize)]
pub struct IssueRequest {
    pub issuer: String,
    pub audience: String,
    pub ttl_sec: u64,
}

#[derive(Serialize)]
pub struct IssueResponse {
    pub token: String,
    pub exp: u64,
}

#[derive(Deserialize)]
pub struct ValidateRequest {
    pub token: String,
    pub self_name: String,
}

#[derive(Serialize)]
pub struct ValidateResponse {
    pub active: bool,
}
