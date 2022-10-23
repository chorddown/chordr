use crate::RecordTrait;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait BackendTrait<R: RecordTrait + Serialize + DeserializeOwned, E, C>:
    super::CommandExecutor<RecordType = R, Error = E, Context = C>
    + super::QueryExecutor<RecordType = R, Error = E, Context = C>
{
}
