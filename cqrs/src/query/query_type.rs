use serde::{Deserialize, Serialize};

/// The `QueryType` describes the query to perform
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryType {
    All,
    ById,
}
