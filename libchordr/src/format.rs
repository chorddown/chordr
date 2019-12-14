#[derive(Copy, Clone)]
pub enum Format {
    HTML,
    #[cfg(feature = "pdf")]
    PDF,
}
