#  hyperdav
A basic, simple to use WebDAV client library.

# Example
Here is how you would put a file on the server.

```rust
extern crate hyperdav;

use std::fs::OpenOptions;

use hyperdav::ClientBuilder;

fn main() {
    let client = ClientBuilder::default()
        .credentials(
            "username",
            "password",
        )
        .build("webdav_url")
        .unwrap();
    let f = OpenOptions::new()
                .read(true)
                .open("/foo/bar/file.txt")
                .unwrap();

    client.put(f, "file.txt");
}
```
