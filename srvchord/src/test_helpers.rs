use crate::DbConn;
use parking_lot::Mutex;
use rocket::local::Client;

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
        // assert!(
        //     Task::delete_all(&$conn),
        //     "Failed to delete all tasks for testing"
        // );

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
        F: Fn(DbConn) -> (),
{
    let _lock = crate::tests::DB_LOCK.lock();
    let rocket = rocket::ignite().attach(DbConn::fairing());
    let db = crate::DbConn::get_one(&rocket);
    let conn = db.expect("Failed to get database connection for testing");

    test_body(conn)
}
