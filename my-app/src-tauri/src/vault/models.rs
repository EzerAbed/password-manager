use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl VaultEntry {
    pub fn new(title: String, username: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            username,
            password, 
            url: None,
            notes: None,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<VaultEntry>,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}