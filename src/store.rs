use crate::models::TokenEntry;
use dashmap::DashMap;
use std::sync::Arc;

pub type TokenStore = Arc<DashMap<String, TokenEntry>>;

pub fn new_store() -> TokenStore {
    Arc::new(DashMap::new())
}
