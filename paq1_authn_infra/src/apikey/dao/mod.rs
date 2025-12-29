use crate::apikey::dbo::ApiKeyDBO;
use async_trait::async_trait;
use paq1_lib_error_handler::prelude::{Error, ErrorWithCode, ResultErr};
use paq1_storage_core::data::quick_search::QuickSearchPath;
use paq1_storage_core::prelude::{DAO, Expression, ExpressionT, Filter, Operation, Query};
use paq1_storage_core::query::Pager;
use paq1_storage_infra::daos::mongo::mongo_dao::MongoDao;

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
        let query = Query {
            pager: Pager::default(),
            filter: Filter::Expression(Expression::ExpressionString(ExpressionT {
                field_name: "hash_key".to_string(),
                operation: Operation::EqualsTo,
                head: pwd.to_string(),
            })),
        };

        self.fetch_all(&query).await.and_then(|list| {
            if list.len() > 1 {
                Err(Error::Failure(ErrorWithCode::new(
                    "DUPKEY",
                    500,
                    "il ne peut pas y avoir deux clef identique en base",
                )))
            } else {
                Ok(list.first().map(|u| u.clone()))
            }
        })
    }
}

#[async_trait]
impl DAO<ApiKeyDBO, String, Error> for MongoApiKeyDao {
    async fn fetch_one(&self, id: &String) -> Result<Option<ApiKeyDBO>, Error> {
        self.underlying.fetch_one(id).await.map_err(|err| {
            Error::Failure(ErrorWithCode::new("REDERR", 500, format!("{err}").as_str()))
        })
    }

    async fn fetch_all(&self, query: &Query) -> Result<Vec<ApiKeyDBO>, Error> {
        self.underlying.fetch_all(query).await.map_err(|err| {
            Error::Failure(ErrorWithCode::new("REDERR", 500, format!("{err}").as_str()))
        })
    }

    async fn insert(&self, entity: &ApiKeyDBO) -> Result<String, Error> {
        self.underlying.insert(entity).await.map_err(|err| {
            Error::Failure(ErrorWithCode::new("REDERR", 500, format!("{err}").as_str()))
        })
    }

    async fn update(&self, entity: &ApiKeyDBO) -> Result<String, Error> {
        self.underlying.update(entity).await.map_err(|err| {
            Error::Failure(ErrorWithCode::new("REDERR", 500, format!("{err}").as_str()))
        })
    }

    async fn delete(&self, id: &String) -> Result<(), Error> {
        self.underlying.delete(id).await.map_err(|err| {
            Error::Failure(ErrorWithCode::new("REDERR", 500, format!("{err}").as_str()))
        })
    }

    async fn delete_all(&self) -> Result<(), Error> {
        self.underlying.delete_all().await.map_err(|err| {
            Error::Failure(ErrorWithCode::new("REDERR", 500, format!("{err}").as_str()))
        })
    }

    async fn quick_search(
        &self,
        _chaine: &str,
        _paths: Vec<QuickSearchPath>,
    ) -> Result<Vec<ApiKeyDBO>, Error> {
        Err(Error::Failure(ErrorWithCode::new(
            "QUSERR",
            501,
            "quick search not implemented for apikey",
        )))
    }

    async fn count(&self) -> Result<u64, Error> {
        self.underlying.count().await.map_err(|err| {
            Error::Failure(ErrorWithCode::new("REDERR", 500, format!("{err}").as_str()))
        })
    }
}
