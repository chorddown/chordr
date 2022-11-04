mod test_value;

use crate::command_context::CommandContext;
use libchordr::prelude::*;
pub use test_value::TestValue;
use webchordr_common::constants::{STORAGE_V2_KEY_SETLIST, TEST_STORAGE_NAMESPACE};

pub struct TestSong {
    id: String,
}

impl SongIdTrait for TestSong {}

impl ListEntryTrait for TestSong {
    type Id = SongId;
    fn id(&self) -> SongId {
        self.id.as_str().into()
    }
}

impl SongData for TestSong {
    fn title(&self) -> String {
        self.id.clone()
    }

    fn file_type(&self) -> FileType {
        FileType::Chorddown
    }
}

pub(super) fn get_test_command_context() -> CommandContext {
    CommandContext::new(TEST_STORAGE_NAMESPACE, STORAGE_V2_KEY_SETLIST)
}
