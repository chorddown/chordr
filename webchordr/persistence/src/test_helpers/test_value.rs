use serde::{Deserialize, Serialize};

use libchordr::prelude::RecordTrait;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TestValue {
    pub age: i32,
    pub name: String,
}
impl TestValue {
    pub fn new<S: Into<String>>(age: i32, name: S) -> Self {
        Self {
            age,
            name: name.into(),
        }
    }
}
impl RecordTrait for TestValue {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.name.clone()
    }
}

impl<'a> RecordTrait for &TestValue {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.name.clone()
    }
}
