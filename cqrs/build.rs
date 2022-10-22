use libcqrs_desync::error::Error;
use libcqrs_desync::prelude::*;
use std::path::PathBuf;

fn main() {
    let base_dir = env!("CARGO_MANIFEST_DIR");

    let files_to_patch = [
        (
            format!("{}/src/nonblocking/repository.rs", base_dir),
            format!("{}/src/blocking/repository.rs", base_dir),
        ),
        (
            format!("{}/src/nonblocking/command_executor.rs", base_dir),
            format!("{}/src/blocking/command_executor.rs", base_dir),
        ),
    ];

    for (source, target) in files_to_patch {
        if let Err(e) = patch_file(&source, &target) {
            println!(
                "cargo:warning=Async/Sync file could not be patched (source: '{}', target: '{}'): {}",
                source,
                target,
                e
            );
        }
    }
}

fn patch_file(source: &str, target: &str) -> Result<(), Error> {
    let file_reader = FileReader::new();
    let file_writer = FileWriter::new();
    let code_updater = CodeUpdater::new();
    let source = PathBuf::from(source)
        .canonicalize()
        .map_err(|e| Error::Path("Could not open source path for reading", Some(e)))?;
    let target = PathBuf::from(target);

    if source.eq(&target) {
        return Err(Error::Path("Source and target are equal", None));
    }

    let mut buffer = file_reader.open_file(&source)?;
    let prepared_content = code_updater.update_code(&mut buffer)?;
    let _ = file_writer.write_generated_file(&prepared_content, &target)?;

    Ok(())
}
