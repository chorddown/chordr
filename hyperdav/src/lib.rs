#![deny(warnings)]
#![deny(missing_debug_implementations)]

use std::string::ToString;

pub use reqwest::blocking::Response;

pub use crate::client::{Client, ClientBuilder};
pub use crate::error::Error;
pub use crate::response::PropfindResponse;

mod client;
mod error;
mod response;

#[derive(Debug)]
/// Define the depth to which we should search.
pub enum Depth {
    /// Any depth you want
    Number(u32),
    /// As deep as we can go
    Infinity,
}

impl ToString for Depth {
    fn to_string(&self) -> String {
        match *self {
            Depth::Number(depth) => depth.to_string(),
            Depth::Infinity => "Infinity".to_string(),
        }
    }
}
