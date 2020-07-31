mod propfind_parser;

use self::propfind_parser::parse_propfind_response;
use crate::error::{Error, Result};
use crate::service::file_entry::FileEntry;
use crate::service::{
    AbstractServiceConfig, ServiceConfigurationTrait, ServiceIdentifier, ServiceTrait,
};
use hyperdav::Client;
use reqwest::{Method, Response, StatusCode, Url};
use std::borrow::Cow;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct WebDAVService {
    client: Client,
    url: Url,
    remote_directory: CowStr,
}

type CowStr = Cow<'static, str>;

impl WebDAVService {
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
        let url_as_string = self.url.to_string();
        let url_parts: Vec<&str> = url_as_string
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

pub struct WebDAVServiceConfiguration {
    url: CowStr,
    remote_directory: CowStr,
    username: CowStr,
    password: CowStr,
    local_directory: PathBuf,
}

impl ServiceConfigurationTrait for WebDAVServiceConfiguration {
    fn from_service_config(service_config: AbstractServiceConfig) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            url: service_config.url()?.into(),
            remote_directory: service_config.remote_directory()?.into(),
            username: service_config.username()?.into(),
            password: service_config.password()?.into(),
            local_directory: service_config.local_directory().to_path_buf(),
        })
    }

    fn identifier(&self) -> ServiceIdentifier {
        ServiceIdentifier::WebDAV
    }

    fn local_directory(&self) -> &Path {
        self.local_directory.as_path()
    }
}

impl ServiceTrait for WebDAVService {
    type Configuration = WebDAVServiceConfiguration;

    fn new(configuration: Self::Configuration) -> Result<Self, Error> {
        let url: Url = Url::parse(configuration.url.as_ref())?;
        let client_result = Client::new()
            .credentials(configuration.username, configuration.password)
            .build(url.as_ref());
        let client = match client_result {
            Ok(c) => c,
            Err(e) => return Err(Error::invalid_argument_error(format!("{}", e))),
        };

        Ok(Self {
            url,
            client,
            remote_directory: configuration.remote_directory,
        })
    }

    fn identifier(&self) -> ServiceIdentifier {
        ServiceIdentifier::WebDAV
    }

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
