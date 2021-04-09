use libchordr::prelude::{RecordIdTrait, RecordTrait};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TestValue {
    pub age: i32,
    pub name: String,
}

impl RecordTrait for TestValue {
    type Id = String;

    fn id(self) -> Self::Id {
        self.name.clone()
    }
}
