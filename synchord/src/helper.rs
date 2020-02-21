use chrono::{DateTime, Utc};
use crate::error::{Error, Result};
use crate::service::*;
use std::path::{Path, PathBuf};
use log::{debug, info, warn, error};


pub fn download(service: &Services, service_config: &ServiceConfig) -> Result<Vec<FileEntry>> {
    let files = service.list_files()?;
    if files.len() == 0 {
        info!("No files found");
    }
    for file in &files {
        let destination = destination_for_file(&file.path(), service_config)?;
        if let Err(e) = check_if_should_download(file, &destination) {
            warn!("Skip download file {}: {}", file.path(), e)
        } else {
            match service.download(file.clone(), &destination) {
                Ok(_) => info!("Downloaded file {}", file.path()),
                Err(e) => error!("Could not download file {}: {}", file.path(), e),
            }
        }
    }
    Ok(files)
}

pub fn check_if_should_download(source: &FileEntry, destination: &Path) -> Result<()> {
    if !(destination.exists()) {
        return Ok(());
    }

    match destination.metadata() {
        Err(_) => Err(Error::download_error("Could not fetch metadata")),
        Ok(metadata) => match metadata.modified() {
            Err(_) => Err(Error::download_error("Could not fetch modification time")),
            Ok(modified) => {
                let remote_time = source.modified_date();
                let local_time: DateTime<Utc> = DateTime::from(modified);
                let local_time_utc = local_time.with_timezone(&remote_time.timezone());

                debug!(
                    "Compare remote vs local file time: {} vs {}",
                    remote_time, local_time_utc
                );
                if local_time_utc < remote_time {
                    info!("Remote file is newer than local file, will overwrite");
                    Ok(())
                } else {
                    Err(Error::skip_download("Local file is newer than remote"))
                }
            }
        },
    }
}

fn destination_for_file<P: AsRef<Path>, S: ServiceConfigTrait>(file: &P, service_config: &S) -> Result<PathBuf> {
    let output_path = get_output_path(service_config)?;

    match file.as_ref().file_name() {
        Some(file_name) => Ok(output_path.join(file_name)),
        None => Err(Error::io_error(format!(
            "Could not get remove name of file {}",
            file.as_ref().to_string_lossy()
        ))),
    }
}

fn get_output_path<S: ServiceConfigTrait>(service_config: &S) -> Result<PathBuf> {
    let output_path = service_config.local_directory()?;
    if output_path.is_dir() {
        return Ok(output_path);
    }

    let output_path_string = output_path.to_str().map_or_else(
        || format!("{}", output_path.to_string_lossy()),
        |s| s.to_owned(),
    );

    if output_path.is_file() {
        Err(Error::io_error(format!(
            "Output path {} is not a directory",
            output_path_string
        )))
    } else if !output_path.exists() {
        Err(Error::io_error(format!(
            "Output path {} does not exist",
            output_path_string
        )))
    } else {
        Err(Error::io_error(format!(
            "Output path {} is not a path",
            output_path_string
        )))
    }
}
