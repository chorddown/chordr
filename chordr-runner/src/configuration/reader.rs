use crate::configuration::Configuration;
use crate::error::*;
use std::error::Error as StdError;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Reader {}

impl Reader {
    pub fn read_configuration_from_file(path: &Path) -> Result<Configuration, Error> {
        match path.extension() {
            None => Err(build_file_type_error(path)),
            Some(os_str) => match os_str.to_str() {
                None => Err(build_file_type_error(path)),

                Some("json") => Reader::read_configuration_from_json_file(path),

                #[cfg(feature = "yaml")]
                Some("yaml") => Reader::read_configuration_from_yaml_file(path),

                Some(t) => Err(Error::configuration_reader_error(format!(
                    "No deserializer for the file type '{}'",
                    t
                ))),
            },
        }
    }

    fn read_configuration_from_json_file(path: &Path) -> Result<Configuration, Error> {
        let file: BufReader<File> = get_file_reader(path)?;
        match serde_json::from_reader::<BufReader<File>, Configuration>(file) {
            Ok(r) => Ok(r),
            Err(e) => Err(build_deserialize_error(path, &e)),
        }
    }

    #[cfg(feature = "yaml")]
    fn read_configuration_from_yaml_file(path: &Path) -> Result<Configuration, Error> {
        let file: BufReader<File> = get_file_reader(path)?;
        match serde_yaml::from_reader::<BufReader<File>, Configuration>(file) {
            Ok(r) => Ok(r),
            Err(e) => Err(build_deserialize_error(path, &e)),
        }
    }
}

fn build_file_type_error(path: &Path) -> Error {
    match path.to_str() {
        None => Error::configuration_reader_error("Invalid file"),
        Some(f) => {
            Error::configuration_reader_error(format!("Could not detect the file type of '{}'", f))
        }
    }
}

fn build_deserialize_error(path: &Path, error: &dyn StdError) -> Error {
    match path.to_str() {
        None => Error::configuration_error(format!("Could not deserialize file: {}", error)),
        Some(f) => {
            Error::configuration_error(format!("Could not deserialize the file '{}': {}", f, error))
        }
    }
}

fn get_file_reader(path: &Path) -> Result<BufReader<File>, Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => Err(Error::configuration_reader_error(format!(
            "Could not open file {:?} for reading: {}",
            path, e
        )))?,
    };
    Ok(BufReader::new(file))
}

#[cfg(test)]
mod test {
    use super::*;
    use libsynchord::prelude::ServiceIdentifier;

    fn assert_valid_mandatory_configuration(result: Result<Configuration, Error>) -> Configuration {
        let configuration = result.unwrap();
        assert_eq!(
            configuration.catalog_file.to_string_lossy(),
            "/tmp/path/to/catalog-file.json"
        );
        assert_eq!(
            configuration.output_directory.to_string_lossy(),
            "/tmp/path/to/download/chorddown-files"
        );
        configuration
    }

    fn assert_valid_configuration(result: Result<Configuration, Error>) {
        assert!(result.is_ok(), result.unwrap_err().to_string());

        let configuration = assert_valid_mandatory_configuration(result);
        assert_valid_webdav_configuration_values(configuration.clone());
        assert_eq!(configuration.service.identifier, ServiceIdentifier::WebDAV);
        assert_eq!(configuration.service.api_token.unwrap(), "MY_API_TOKEN");
    }

    fn assert_valid_dropbox_configuration(result: Result<Configuration, Error>) {
        assert!(result.is_ok(), result.unwrap_err().to_string());

        let configuration = assert_valid_mandatory_configuration(result);
        assert_eq!(configuration.service.identifier, ServiceIdentifier::Dropbox);
        assert_eq!(configuration.service.api_token.unwrap(), "MY_API_TOKEN");
    }

    fn assert_valid_webdav_configuration(result: Result<Configuration, Error>) {
        assert!(result.is_ok(), result.unwrap_err().to_string());

        let configuration = assert_valid_mandatory_configuration(result);
        assert_valid_webdav_configuration_values(configuration);
    }

    fn assert_valid_webdav_configuration_values(configuration: Configuration) {
        assert_eq!(configuration.service.identifier, ServiceIdentifier::WebDAV);
        assert_eq!(configuration.service.username.unwrap(), "this-is-me");
        assert_eq!(configuration.service.password.unwrap(), "123-easy");
        assert_eq!(
            configuration.service.url.unwrap(),
            "https://mycloud.example.com"
        );
        assert_eq!(
            configuration.service.remote_directory.unwrap(),
            "remote-dir"
        );
        assert_eq!(configuration.service.sync_interval, 34);
    }

    #[test]
    fn read_configuration_from_file_invalid() {
        let result = Reader::read_configuration_from_file(&Path::new("/tests/"));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Configuration reader error: Could not detect the file type of '/tests/'"
        );

        let result = Reader::read_configuration_from_file(&Path::new(&format!(
            "{}/tests.txt",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Configuration reader error: No deserializer for the file type 'txt'"
        );
    }

    #[test]
    fn read_configuration_from_file_with_not_existing_json() {
        let result =
            Reader::read_configuration_from_file(&Path::new("/tests/resources/not-a-file.json"));
        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().starts_with(
                "Configuration reader error: Could not open file \"/tests/resources/not-a-file.json\" for reading: No such file or directory"
            ));
    }

    #[test]
    fn read_configuration_from_file_with_json() {
        let result = Reader::read_configuration_from_file(&Path::new(&format!(
            "{}/tests/resources/configuration.json",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert_valid_configuration(result);
    }

    #[test]
    fn read_dropbox_configuration_from_file() {
        let result = Reader::read_configuration_from_file(&Path::new(&format!(
            "{}/tests/resources/configuration-dropbox.json",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert_valid_dropbox_configuration(result);
    }

    #[test]
    fn read_webdav_configuration_from_file() {
        let result = Reader::read_configuration_from_file(&Path::new(&format!(
            "{}/tests/resources/configuration-webdav.json",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert_valid_webdav_configuration(result);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn read_configuration_from_file_with_not_existing_yaml() {
        let result = Reader::read_configuration_from_file(&Path::new(
            "/tests/resources/not-found-configuration.yaml",
        ));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().starts_with("Configuration reader error: Could not open file \"/tests/resources/not-found-configuration.yaml\" for reading: No such file or directory"));
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn read_configuration_from_file_with_yaml() {
        let result = Reader::read_configuration_from_file(&Path::new(&format!(
            "{}/tests/resources/configuration.yaml",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert_valid_configuration(result);
    }
}
