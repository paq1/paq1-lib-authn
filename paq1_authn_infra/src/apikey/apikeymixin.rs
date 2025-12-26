use std::sync::Arc;
use paq1_authn_core::apikey::services::context_provider::ContextProvider;
use paq1_authn_core::apikey::services::password_hasher::PasswordHasher;
use crate::apikey::dao::ApiKeyDAO;
use crate::apikey::middleware::ApiKeyAuth;
use crate::apikey::services::context_provider::DefaultContextProvider;
use crate::apikey::services::password_hasher::DefaultPasswordHasher;

pub struct ApiKeyMixin {
    pub middleware: Arc<ApiKeyAuth>,
    pub context_provider: Arc<dyn ContextProvider>,
}

impl ApiKeyMixin {
    pub async fn new(api_key_dao: Arc<dyn ApiKeyDAO>, salt: &str) -> Self {

        let password_hasher: Arc<dyn PasswordHasher> = Arc::new(DefaultPasswordHasher::new(salt));
        let context_provider = Arc::new(DefaultContextProvider::new(password_hasher.clone(), api_key_dao.clone()));

        Self {
            middleware: Arc::new(
                ApiKeyAuth {
                    context_provider: context_provider.clone()
                }
            ),
            context_provider,
        }

    }
}
