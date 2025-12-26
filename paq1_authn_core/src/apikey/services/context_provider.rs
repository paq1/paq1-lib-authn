use async_trait::async_trait;
use paq1_lib_error_handler::prelude::ResultErr;
use crate::data::context::ContextCore;

#[async_trait]
pub trait ContextProvider: Send + Sync {
    async fn build_context(&self, pure: &str) -> ResultErr<ContextCore>;
}
