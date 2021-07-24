use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    if let Ok(n) = SystemTime::now().duration_since(UNIX_EPOCH) {
        println!("cargo:rustc-env=CUNDD_BUILD_REVISION={}", n.as_secs())
    }
}
