use paq1_storage_infra::daos::mongo::identifier::HasIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyDBO {
    pub hash_key: String,
    pub user_id: String,
    pub active: bool,
}
impl ApiKeyDBO {
    pub fn get_subject(&self) -> &str {
        self.user_id.as_str()
    }
}

impl HasIdentifier for ApiKeyDBO {
    fn identifier_value(&self) -> &String {
        &self.hash_key
    }

    fn identifier_key() -> String {
        "hash_key".to_string()
    }
}
