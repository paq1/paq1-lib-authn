use async_trait::async_trait;
use paq1_lib_error_handler::prelude::{Error, ErrorWithCode, ResultErr};
use crate::apikey::dbo::ApiKeyDBO;
use paq1_storage_core::prelude::{Query, DAO};
use paq1_storage_infra::daos::mongo::mongo_dao::MongoDao;
use mongodb::bson::doc;
use paq1_storage_core::data::quick_search::QuickSearchPath;

#[async_trait]
pub trait ApiKeyDAO: DAO<ApiKeyDBO, String, Error> + Send + Sync {
    async fn fetch_one_from_pwd(&self, pwd: &str) -> ResultErr<Option<ApiKeyDBO>>;
}
pub struct MongoApiKeyDao {
    pub underlying: MongoDao<ApiKeyDBO>,
}

#[async_trait]
impl ApiKeyDAO for MongoApiKeyDao {
    async fn fetch_one_from_pwd(&self, pwd: &str) -> ResultErr<Option<ApiKeyDBO>> {
        let filter = doc! {
            "hash_key": pwd
        };

        self.underlying.collection
            .find_one(filter)
            .await
            .map_err(|e| Error::Failure(ErrorWithCode {
                code: "AKFOER".to_string(),
                status: 500,
                title: e.to_string(),
                description: None,
                problems: vec![]
            }))
    }
}

#[async_trait]
impl DAO<ApiKeyDBO, String, Error> for MongoApiKeyDao {
    async fn fetch_one(&self, id: &String) -> Result<Option<ApiKeyDBO>, Error> {
        self.underlying.fetch_one(id).await.map_err(|err| Error::Failure(ErrorWithCode::new("REDERR", 500,format!("{err}").as_str())))
    }

    async fn fetch_all(&self, query: &Query) -> Result<Vec<ApiKeyDBO>, Error> {
        self.underlying.fetch_all(query).await.map_err(|err| Error::Failure(ErrorWithCode::new("REDERR", 500,format!("{err}").as_str())))
    }

    async fn insert(&self, entity: &ApiKeyDBO) -> Result<String, Error> {
        self.underlying.insert(entity).await.map_err(|err| Error::Failure(ErrorWithCode::new("REDERR", 500,format!("{err}").as_str())))
    }

    async fn update(&self, entity: &ApiKeyDBO) -> Result<String, Error> {
        self.underlying.update(entity).await.map_err(|err| Error::Failure(ErrorWithCode::new("REDERR", 500,format!("{err}").as_str())))
    }

    async fn delete(&self, id: &String) -> Result<(), Error> {
        self.underlying.delete(id).await.map_err(|err| Error::Failure(ErrorWithCode::new("REDERR", 500,format!("{err}").as_str())))
    }

    async fn delete_all(&self) -> Result<(), Error> {
        self.underlying.delete_all().await.map_err(|err| Error::Failure(ErrorWithCode::new("REDERR", 500,format!("{err}").as_str())))
    }

    async fn quick_search(&self, _chaine: &str, _paths: Vec<QuickSearchPath>) -> Result<Vec<ApiKeyDBO>, Error> {
        Err(Error::Failure(ErrorWithCode::new("QUSERR", 501, "quick search not implemented for apikey")))
    }

    async fn count(&self) -> Result<u64, Error> {
        self.underlying.count().await.map_err(|err| Error::Failure(ErrorWithCode::new("REDERR", 500,format!("{err}").as_str())))
    }
}
