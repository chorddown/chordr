use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    /// Path to the directory of chorddown files
    pub song_dir: String,

    /// Path to the static files (e.g. stylesheets, JavaScript, images)
    pub static_files_dir: String,
}
