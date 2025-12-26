use crate::apikey::dao::ApiKeyDAO;
use async_trait::async_trait;
use paq1_authn_core::apikey::services::context_provider::ContextProvider;
use paq1_authn_core::apikey::services::password_hasher::PasswordHasher;
use paq1_authn_core::data::context::ContextCore;
use paq1_lib_error_handler::prelude::{Error, ErrorWithCode, ResultErr};
use std::sync::Arc;

pub struct DefaultContextProvider {
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub dao: Arc<dyn ApiKeyDAO>
}

impl DefaultContextProvider {
    pub fn new(password_hasher: Arc<dyn PasswordHasher>, dao: Arc<dyn ApiKeyDAO>) -> Self {
        Self { password_hasher, dao }
    }
}

#[async_trait]
impl ContextProvider for DefaultContextProvider {
    async fn build_context(&self, pure: &str) -> ResultErr<ContextCore> {
        let key = self.password_hasher.hashed(pure)?;

        let dbo = self.dao
            .fetch_one_from_pwd(key.as_str())
            .await?
            .ok_or(Error::Failure(ErrorWithCode {
                code: "AKNFERR".to_string(),
                status: 401,
                title: "la clef api n'existe pas".to_string(),
                description: None,
                problems: vec![],
            }))?;

        Ok(ContextCore::from_now(dbo.get_subject(), None)) // MKDMKD : ne pas mettre de token dans le cadre d'api key
    }
}