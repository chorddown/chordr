use crate::WebError;

/// Trait mirroring the API for Browser Storage
///
/// https://developer.mozilla.org/en-US/docs/Web/API/Storage
pub trait BrowserStorageTrait {
    // /// When passed a number n, this method will return the name of the nth key in the storage
    // fn key(&self, index: usize) -> Option<String>;

    /// When passed a key name, will return that key's value
    fn get_item<S: AsRef<str>>(&self, key_name: S) -> Option<String>;

    /// When passed a key name and value, will add that key to the storage, or update that key's value if it already exists
    fn set_item<S: Into<String>, V: Into<String>>(
        &mut self,
        key_name: S,
        key_value: V,
    ) -> Result<(), WebError>;

    /// When passed a key name, will remove that key from the storage
    fn remove_item<S: AsRef<str>>(&mut self, key_name: S) -> Result<(), WebError>;

    /// When invoked, will empty all keys out of the storage
    fn clear(&mut self) -> Result<(), WebError>;

    /// Return the number of pairs in the storage
    fn len(&self) -> usize;
}
