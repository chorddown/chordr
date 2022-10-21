use crate::error::Error;
use chrono::Local;
use std::io::Read;

pub struct CodeUpdater {}

impl CodeUpdater {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update_code(&self, input: &mut impl Read) -> Result<String, Error> {
        let prepared_content = self.remove_async_keywords(input)?;

        let output = format!(
            "/// This file was auto-generated by {} on {}\n/// Do not edit it\n\n{}",
            env!("CARGO_PKG_NAME"),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            prepared_content
        );

        Ok(output)
    }

    fn remove_async_keywords(&self, input: &mut impl Read) -> Result<String, Error> {
        let mut buffer = String::new();
        if let Err(e) = input.read_to_string(&mut buffer) {
            return Err(Error::Read("Could not read the reader's content", e));
        }

        let delete_patterns = [
            "use async_trait::async_trait;",
            "#[async_trait(? Send)]",
            "#[async_trait(?Send)]",
            "#[async_trait()]",
            "#[async_trait]",
        ];

        let buffer = delete_patterns
            .into_iter()
            .fold(buffer, |acc, item| acc.replace(item, ""));
        Ok(buffer.replace(" async fn ", " fn "))
    }
}
