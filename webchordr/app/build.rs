use chrono::Local;

fn main() {
    println!(
        "cargo:rustc-env=CUNDD_BUILD_REVISION={}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    )
}
