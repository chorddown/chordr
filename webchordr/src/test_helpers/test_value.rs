use libchordr::prelude::RecordIdTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TestValue {
    pub age: i32,
    pub name: String,
}

impl RecordIdTrait for TestValue {
    type Id = String;

    fn id(self) -> Self::Id {
        self.name.clone()
    }
}
