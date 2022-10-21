use clap::Parser;
use libcqrs_desync::code_updater::CodeUpdater;
use libcqrs_desync::error::Error;
use libcqrs_desync::file_reader::FileReader;
use libcqrs_desync::file_writer::FileWriter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    source: String,
    target: String,
    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        if let Some(source) = std::error::Error::source(&e) {
            eprintln!("{}", source);
        }
    }
}
fn run() -> Result<(), Error> {
    let args: Args = Args::parse();

    let file_reader = FileReader::new();
    let file_writer = FileWriter::new();
    let code_updater = CodeUpdater::new();

    let source = PathBuf::from(args.source)
        .canonicalize()
        .map_err(|e| Error::Path("Could not open source path for reading", Some(e)))?;
    let target = PathBuf::from(args.target);
    if args.verbose {
        println!(
            "Patch \n   {}\nand save in \n   {}",
            source.display(),
            target.display()
        );
    }
    let mut buffer = file_reader.open_file(&source)?;
    let prepared_content = code_updater.update_code(&mut buffer)?;
    let _ = file_writer.write_generated_file(&prepared_content, &target)?;

    if args.verbose {
        println!(
            "--------------------------------------------------------------------------------"
        );
        println!("{}", prepared_content);
        println!(
            "--------------------------------------------------------------------------------"
        );
    }

    Ok(())
}
// fn run() -> Result<(), libcqrs_desync::error::Error> {
//     let args = Args::parse();
//
//     let base_dir = format!("{}/../", env!("CARGO_MANIFEST_DIR"));
//     let mapping = [
//         (
//             format!("{}{}", base_dir, "cqrs/src/async_test.rs"),
//             format!("{}{}", base_dir, "cqrs/src/sync_test.rs"),
//         ),
//         (
//             format!("{}{}", base_dir, "cqrs/src/async_repository.rs"),
//             format!("{}{}", base_dir, "cqrs/src/sync_repository.rs"),
//         ),
//     ];
//     let file_reader = FileReader::new();
//     let file_writer = FileWriter::new();
//     let code_updater = CodeUpdater::new();
//     for (source, target) in mapping {
//         let source = PathBuf::from(source).canonicalize()?;
//         let target = PathBuf::from(target);
//         if args.verbose {
//             println!(
//                 "Analyse \n   {}\nand save in \n   {}",
//                 source.display(),
//                 target.display()
//             );
//         }
//         let mut buffer = file_reader.open_file(&source)?;
//         let prepared_content = code_updater.update_code(&mut buffer)?;
//         let _ = file_writer.write_generated_file(&prepared_content, &target)?;
//
//         if args.verbose {
//             println!(
//                 "--------------------------------------------------------------------------------"
//             );
//             println!("{}", prepared_content);
//             println!(
//                 "--------------------------------------------------------------------------------"
//             );
//         }
//     }
//
//     Ok(())
// }
