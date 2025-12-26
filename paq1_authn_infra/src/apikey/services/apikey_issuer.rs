use crate::apikey::dao::ApiKeyDAO;
use async_trait::async_trait;
use paq1_lib_error_handler::prelude::{Error, ErrorWithCode, ResultErr};
use std::sync::Arc;
use paq1_authn_core::apikey::services::password_hasher::PasswordHasher;
use crate::apikey::dbo::ApiKeyDBO;

#[async_trait]
pub trait ApiKeyIssuer {
    async fn create_api_key(&self, user: &str) -> ResultErr<String>;
}

pub struct DefaultApiKeyIssuer {
    dao: Arc<dyn ApiKeyDAO>,
    password_hasher: Arc<dyn PasswordHasher>
}

impl DefaultApiKeyIssuer {
    pub fn new(dao: Arc<dyn ApiKeyDAO>, password_hasher: Arc<dyn PasswordHasher>) -> Self {
        Self { dao, password_hasher }
    }
}

#[async_trait]
impl ApiKeyIssuer for DefaultApiKeyIssuer {
    async fn create_api_key(&self, user: &str) -> ResultErr<String> {
        let maybe_already_exist = self.dao.fetch_one(&user.to_string()).await?;
        if let Some(_) = maybe_already_exist {
            Err(Error::Failure(ErrorWithCode {
                code: "KAEERR".to_string(),
                status: 400,
                title: format!("le user {user} existe déjà"),
                description: None,
                problems: vec![],
            }))
        } else {
            let pure = self.password_hasher.generate_pure_random()?;
            let hashed = self.password_hasher.hashed(pure.as_str())?;
            self.dao
                .insert(&ApiKeyDBO {
                    hash_key: hashed,
                    user_id: user.to_string(),
                    active: true,
                })
                .await?;
            Ok(pure)
        }
    }
}
