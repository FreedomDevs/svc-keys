use std::collections::HashMap;
use std::sync::Arc;

pub type Permissions = Arc<HashMap<String, Vec<String>>>;

pub fn load_permissions_embedded() -> Permissions {
    let data = include_str!("../permissions.json");

    let map: HashMap<String, Vec<String>> =
        serde_json::from_str(data).expect("Failed to parse embedded JSON permissions");

    Arc::new(map)
}

pub fn is_allowed(perms: &Permissions, issuer: &str, audience: &str) -> bool {
    perms
        .get(issuer)
        .map(|list| list.contains(&audience.to_string()))
        .unwrap_or(false)
}
