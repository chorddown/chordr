use std::borrow::Cow;

use reqwest::blocking::{ Client as HttpClient, RequestBuilder};
use reqwest::Url;
use reqwest::{IntoUrl, Method};

use crate::error::Result;

type CowStr = Cow<'static, str>;

/// The WebDAV client. Make a `Client` for each server.
#[derive(Debug)]
pub struct Client {
    http_client: HttpClient,
    webdav_url: Url,
    credentials: Option<Credentials>,
}

/// The credentials for authenticating with the WebDAV server.
#[derive(Default, Debug)]
pub struct Credentials {
    username: CowStr,
    password: CowStr,
}

/// The builder for the `Client`.
#[derive(Default, Debug)]
pub struct ClientBuilder {
    username: Option<CowStr>,
    password: Option<CowStr>,
}

impl ClientBuilder {
    /// Construct a new `ClientBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the credentials for the server.
    pub fn credentials<S>(mut self, username: S, password: S) -> Self
    where
        S: Into<CowStr>,
    {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Build the WebDAV `Client`.
    ///
    /// # Errors
    ///
    /// This methods fails if the passed url is invalid.
    pub fn build<U>(self, webdav_url: U) -> Result<Client>
    where
        U: IntoUrl,
    {
        let credentials = if let ClientBuilder {
            username: Some(u),
            password: Some(p),
        } = self
        {
            Some(Credentials {
                username: u,
                password: p,
            })
        } else {
            None
        };

        Ok(Client {
            http_client: HttpClient::new(),
            webdav_url: webdav_url.into_url()?,
            credentials,
        })
    }
}

impl Client {
    /// Constructs a new `ClientBuilder`.
    pub fn new() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Prepare a `RequestBuilder` for use in a request to the WebDAV server.
    /// This can be used in case you need to customize something or want to do something which is
    /// still unsupported.
    pub fn request<I>(&self, method: Method, path: I) -> RequestBuilder
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut url = self.webdav_url.clone();
        url.path_segments_mut().unwrap().extend(path);
        let request = self.http_client.request(method, url.as_str());

        if let Some(Credentials {
            ref username,
            ref password,
        }) = self.credentials
        {
            request.basic_auth(username.to_string(), Some(password.to_string()))
        } else {
            request
        }
    }
}
