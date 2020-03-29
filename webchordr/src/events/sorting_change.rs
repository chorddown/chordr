use serde::{Deserialize, Serialize};
use crate::events::EventTrait;

pub type Sorting = usize;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SortingChange {
    old_index: Sorting,
    new_index: Sorting,
}

impl SortingChange {
    pub fn new(old_index: Sorting, new_index: Sorting) -> Self {
        Self {
            old_index,
            new_index,
        }
    }

    pub fn old_index(&self) -> Sorting {
        self.old_index
    }

    pub fn new_index(&self) -> Sorting {
        self.new_index
    }
}

impl EventTrait for SortingChange {}
