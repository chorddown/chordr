mod propfind_parser;

use self::propfind_parser::parse_propfind_response;
use crate::error::{Error, Result};
use crate::service::file_entry::FileEntry;
use crate::service::ServiceTrait;
use hyperdav::Client;
use reqwest::{Method, Response, StatusCode};
use std::borrow::Cow;
use std::fs::File;
use std::path::Path;

pub struct WebDAVService {
    client: Client,
    url: CowStr,
    remote_directory: CowStr,
}

type CowStr = Cow<'static, str>;

impl WebDAVService {
    pub fn new<S: Into<CowStr>, U: Into<CowStr>>(
        url: U,
        remote_directory: S,
        username: S,
        password: S,
    ) -> Result<Self> {
        let url = url.into();
        let client = match Client::new()
            .credentials(username.into(), password.into())
            .build(url.as_ref())
        {
            Ok(c) => c,
            Err(e) => return Err(Error::invalid_argument_error(format!("{}", e))),
        };

        Ok(Self {
            url,
            client,
            remote_directory: remote_directory.into(),
        })
    }

    /// List files in a directory on the WebDAV server.
    ///
    /// This method fails if the passed path doesn't exist on the WebDAV server.
    pub fn list<I>(&self, path: I) -> Result<Vec<FileEntry>>
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
            .client
            .request(Method::Extension("PROPFIND".to_string()), path)
            .body(body)
            .send()?;

        self.check_status_code(&res.status())?;

        let files = parse_propfind_response(res)?;
        Ok(files
            .into_iter()
            .filter(|f| !f.is_directory())
            .map(From::from)
            .collect())
    }

    fn fetch_file(&self, path: String) -> Result<Response> {
        let clean_path = self.remove_overlapping_path_segments(path);
        let res = self.client.request(Method::Get, clean_path).send()?;
        self.check_status_code(&res.status())?;

        Ok(res)
    }

    /// Remove the overlapping parts from the path and the base URL
    fn remove_overlapping_path_segments(&self, path: String) -> Vec<String> {
        let path_parts: Vec<&str> = path
            .split_terminator('/')
            .filter(|s| !s.is_empty())
            .collect();
        let url_parts: Vec<&str> = self
            .url
            .split_terminator('/')
            .filter(|s| !s.is_empty())
            .collect();

        let mut url_parts_iter = url_parts.into_iter();

        let mut clean: Vec<String> = vec![];
        for path_part in path_parts {
            let mut has_match = false;
            while let Some(url_part) = url_parts_iter.next() {
                if url_part == path_part {
                    has_match = true;
                    break;
                }
            }
            if !has_match {
                clean.push(path_part.to_owned())
            }
        }

        clean
    }

    fn check_status_code(&self, status: &StatusCode) -> Result<()> {
        if !status.is_success() {
            Err(Error::download_error(match status.canonical_reason() {
                Some(reason) => format!("{}", reason),
                None => format!("{}", status),
            }))
        } else {
            Ok(())
        }
    }
}

impl ServiceTrait for WebDAVService {
    fn list_files(&self) -> Result<Vec<FileEntry>, Error> {
        self.list(self.remote_directory.split('/').collect::<Vec<&str>>())
    }

    fn download(&self, file: FileEntry, destination: &Path) -> Result<()> {
        let mut res = self.fetch_file(file.path().to_owned())?;
        let mut file_handle = File::create(destination)?;
        res.copy_to(&mut file_handle)?;
        Ok(())
    }
}
