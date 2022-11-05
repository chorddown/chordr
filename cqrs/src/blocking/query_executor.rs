/// This file was auto-generated by cqrs-desync on 2022-11-05 16:40:23
/// Do not edit it

use crate::query::Query;
use crate::RecordTrait;

use serde::de::DeserializeOwned;
use tri::Tri;


pub trait QueryExecutor {
    type RecordType: RecordTrait + DeserializeOwned;
    type Error;
    type Context;

    fn find_all(
        &self,
        query: &Query<Self::RecordType, Self::Context>,
    ) -> Result<Vec<Self::RecordType>, Self::Error>;

    fn find_by_id(
        &self,
        query: &Query<Self::RecordType, Self::Context>,
    ) -> Tri<Self::RecordType, Self::Error>;
}
