#![deny(warnings)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # hyperdav
//! The `hyperdav` crate provides an API for interacting with the WebDAV protocol.
//!
//! It's easy to use and handles all the abstractions over HTTP for the user.
//!
//! ## GET request
//!
//! ```rust
//! # extern crate hyperdav;
//! # use hyperdav::{Client, Error};
//! #
//! # fn run() -> Result<(), Error> {
//! let client = Client::new()
//!     .credentials("foo", "bar")
//!     .build("https://demo.owncloud.org/remote.php/webdav/")
//!     .unwrap();
//!
//! let mut res = client.get(&["file.txt"])?;
//! let mut buf = vec![];
//! res.copy_to(&mut buf)?;
//! # Ok(())
//! # }
//! ```
//!
//! The GET request will return a [`Response`][response] from the [`reqwest`][reqwest] crate on
//! success.
//!
//! ## PUT request
//!
//! ```rust
//! # extern crate hyperdav;
//! # use hyperdav::{Client, Error};
//! #
//! # fn run() -> Result<(), Error> {
//! let client = Client::new()
//!     .credentials("foo", "bar")
//!     .build("https://demo.owncloud.org/remote.php/webdav/")
//!     .unwrap();
//! let r = std::io::empty();
//! client.put(r, &["file.txt"])?;
//!     # Ok(())
//! # }
//!
//! ```
//!
//! The PUT request will return `()` on success just to indicate it succeeded
//!
//! [response]: ./struct.Response.html
//! [reqwest]: https://crates.io/crates/reqwest

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
