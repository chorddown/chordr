use std::borrow::Cow;
use std::io::Read;

use reqwest::blocking::{Body, Client as HttpClient, RequestBuilder, Response};
use reqwest::Url;
use reqwest::{IntoUrl, Method};

use crate::error::Result;
use crate::response::{parse_propfind_response, PropfindResponse};
use crate::Error;

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

    /// Get a file from the WebDAV server.
    ///
    /// # Errors
    ///
    /// This method fails if the passed path doesn't exist on the WebDAV server.
    pub fn get<I>(&self, path: I) -> Result<Response>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let res = self.request(Method::GET, path).send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(res)
    }

    /// Put a file on the WebDAV server, make sure the URL is pointing to the location where you
    /// want the file to be.
    ///
    /// # Errors
    ///
    /// This method fails if the passed path doesn't exist on the WebDAV server.
    pub fn put<R, I>(&self, body: R, path: I) -> Result<()>
    where
        R: Read + Send + 'static,
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let res = self
            .request(Method::PUT, path)
            .body(Body::new(body))
            .send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(())
    }

    /// Creates a directory on the WebDAV server.
    ///
    /// # Errors
    ///
    /// This methods fails if the path where you want to create the directory doesn't exist.
    pub fn mkcol<I>(&self, path: I) -> Result<()>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let res = self
            .request(Method::from_bytes(b"MKCOL").unwrap(), path)
            .send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?
        }

        Ok(())
    }

    /// Rename or move a directory or file on the WebDAV server.
    ///
    /// # Errors
    ///
    /// This method fails if from doesn't exist, also fails if the to path is invalid.
    pub fn mv<F, T>(&self, from: F, to: T) -> Result<()>
    where
        F: IntoIterator,
        F::Item: AsRef<str>,
        T: IntoIterator,
        T::Item: AsRef<str>,
    {
        let mut url = self.webdav_url.clone();
        url.path_segments_mut().unwrap().extend(to);
        let req = self
            .request(Method::from_bytes(b"MOVE").unwrap(), from)
            .header("Destination", url.to_string());

        let res = req.send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(())
    }

    /// List files in a directory on the WebDAV server.
    ///
    /// # Errors
    ///
    /// This method fails if the passed path doesn't exist on the WebDAV server.
    pub fn list<I>(&self, path: I, depth: Option<&str>) -> Result<Vec<PropfindResponse>>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let body = r#"<?xml version="1.0" encoding="utf-8" ?>
            <D:propfind xmlns:D="DAV:">
                <D:allprop/>
            </D:propfind>
        "#;

        let res = self
            .request(Method::from_bytes(b"PROPFIND").unwrap(), path)
            .header("Depth", depth.unwrap_or("Infinity"))
            .body(body)
            .send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(parse_propfind_response(res)?)
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
