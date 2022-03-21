use std::env;
use std::path::PathBuf;
use std::rc::Rc;

use libchordr::prelude::{
    CatalogBuilder, FileType, ListEntryTrait, SearchIndex, SongData, SongSorting,
};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let search_term = get_search_term(&args);
    let path_buf = get_song_directory(&args);
    let verbose = get_verbose(&args);
    let catalog_builder = CatalogBuilder::new();

    println!(
        "Search for '{}' in path {}",
        search_term,
        path_buf.display()
    );
    let catalog = catalog_builder
        .build_catalog_for_directory(path_buf, FileType::Chorddown, false)
        .unwrap()
        .catalog;

    let catalog = Rc::new(catalog);
    let index = SearchIndex::build_for_catalog(catalog);
    let search_results = index.search_by_term(&search_term);
    if verbose {
        println!("{:?}", search_results)
    } else {
        for song in search_results.sort_by_title() {
            println!("{} (ID: '{}')", song.title(), song.id())
        }
    }
}

fn get_verbose(args: &[String]) -> bool {
    args.get(3)
        .map_or(false, |a| if a == "-v" { true } else { false })
}

fn get_search_term(args: &[String]) -> String {
    let default_search_term = format!("grace");

    args.get(1).cloned().unwrap_or(default_search_term)
}

fn get_song_directory(args: &[String]) -> PathBuf {
    let default_path = format!(
        "{}/../webchordr/app/static/songs",
        env!("CARGO_MANIFEST_DIR")
    );
    let path = args.get(2).unwrap_or(&default_path);
    PathBuf::from(&path)
}
