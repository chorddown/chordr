use crate::{DbConn, ConnectionType};
use parking_lot::Mutex;
use rocket::local::Client;
use rocket::config::RocketConfig;
use diesel::Connection;

// We use a lock to synchronize between tests so DB operations don't collide.
// For now. In the future, we'll have a nice way to run each test in a DB
// transaction so we can regain concurrency.
pub(crate) static DB_LOCK: Mutex<()> = Mutex::new(());

#[macro_export]
macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        let _lock = $crate::tests::DB_LOCK.lock();
        let rocket = $crate::rocket();
        let db = $crate::DbConn::get_one(&rocket);
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("Failed to get database connection for testing");

        $block
    }};
}

pub fn run_test_fn<F>(test_body: F) -> ()
    where
        F: Fn(Client, DbConn) -> (),
{
    let _lock = crate::tests::DB_LOCK.lock();
    let rocket = crate::rocket();
    let db = crate::DbConn::get_one(&rocket);
    let client = Client::new(rocket).expect("Rocket client");
    let conn = db.expect("Failed to get database connection for testing");

    test_body(client, conn)
}

pub fn run_database_test<F>(test_body: F) -> ()
    where
        F: Fn(ConnectionType) -> (),
{
    let _lock = crate::tests::DB_LOCK.lock();

    let missing_database_error = "Failed to get database connection for testing";
    let config = RocketConfig::read().unwrap().active().clone();
    let database_url = config
        .get_table("databases")
        .expect(missing_database_error)
        .get("main_database")
        .expect(missing_database_error)
        .get("url")
        .expect(missing_database_error);

    let conn = <ConnectionType as Connection>::establish(&database_url.as_str().unwrap())
        .expect(&format!("Error connecting to {}", database_url));

    test_body(conn)
}
