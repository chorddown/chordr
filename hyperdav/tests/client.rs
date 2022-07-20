use hyperdav::{Client, ClientBuilder};
use uuid::Uuid;

const OWNCLOUD_URL: &'static str = "https://demo.owncloud.org/remote.php/webdav/";

fn get_client() -> Client {
    let uuid = Uuid::new_v4();
    let url = format!("{}{}", OWNCLOUD_URL, uuid);
    ClientBuilder::default()
        .credentials("test", "test")
        .build(&url)
        .unwrap()
}

#[test]
fn put() {
    let client = get_client();
    let f = std::io::empty();
    let put_res = client.put(f, &[""]);

    assert!(put_res.is_ok());
}

#[test]
fn get() {
    let client = get_client();
    let f = std::io::empty();
    let put_res = client.put(f, &[""]);
    let get_res = client.get(&[""]);

    assert!(put_res.is_ok() && get_res.is_ok());
}

#[test]
fn mkcol() {
    let client = get_client();
    let mkcol_res = client.mkcol(&[""]);

    assert!(mkcol_res.is_ok());
}

#[test]
fn mv() {
    let client = get_client();
    let mkcol_res_root = client.mkcol(&[""]);
    let mkcol_res = client.mkcol(&["from"]);
    let mv_res = client.mv(&["from"], &["to"]);

    assert!(mkcol_res_root.is_ok() && mkcol_res.is_ok() && mv_res.is_ok());
}

#[test]
fn list() {
    let client = get_client();
    let mkcol_res = client.mkcol(&[""]);
    let list_res = client.list(&[""], Some("3"));

    assert!(
        mkcol_res.is_ok(),
        "MKCOL failed: {}",
        mkcol_res.unwrap_err()
    );
    assert!(list_res.is_ok(), "LIST failed: {}", list_res.unwrap_err());
}
