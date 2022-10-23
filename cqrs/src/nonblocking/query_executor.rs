use crate::query::Query;
use crate::RecordTrait;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use tri::Tri;

#[async_trait(? Send)]
pub trait QueryExecutor {
    type RecordType: RecordTrait + DeserializeOwned;
    type Error;
    type Context;

    async fn find_all(
        &self,
        query: Query<Self::RecordType, Self::Context>,
    ) -> Result<Vec<Self::RecordType>, Self::Error>;

    async fn find_by_id(
        &self,
        query: Query<Self::RecordType, Self::Context>,
    ) -> Tri<Self::RecordType, Self::Error>;
}
