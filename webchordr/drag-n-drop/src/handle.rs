use super::wasm::DropzoneWrapper;
use crate::OnDropClosure;

#[must_use]
pub struct DropzoneHandle {
    pub wrapper: DropzoneWrapper,
    pub _closure: OnDropClosure,
}

impl DropzoneHandle {
    pub fn destroy(&mut self) {
        self.wrapper.destroy();
    }
}

impl Drop for DropzoneHandle {
    fn drop(&mut self) {
        self.wrapper.destroy();
    }
}
