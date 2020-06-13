mod json_formatting;

use crate::{ConnectionType, DbConn};
use diesel::Connection;
use parking_lot::Mutex;
use rocket::config::RocketConfig;
use rocket::local::Client;
pub use self::json_formatting::*;
use rand::{thread_rng, Rng};
use crate::domain::user::User;
use crate::domain::user::repository::UserRepository;
use crate::traits::RepositoryTrait;
use crate::domain::setlist::UserSetlist;
use libchordr::prelude::{SetlistEntry, FileType};
use crate::command::{CommandExecutor, Command};

#[allow(unused)]
enum UseDatabase {
    InMemory,
    FromConfig,
    FromString,
}

const USE_DATABASE: UseDatabase = UseDatabase::InMemory;

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
    let _lock = crate::test_helpers::DB_LOCK.lock();
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
    let _lock = crate::test_helpers::DB_LOCK.lock();

    let database_url = match USE_DATABASE {
        UseDatabase::InMemory => ":memory:".to_owned(),
        UseDatabase::FromString => "db/test-db.sqlite".to_owned(),
        UseDatabase::FromConfig => {
            let missing_database_error = "Failed to get database connection for testing";
            let config = RocketConfig::read().unwrap().active().clone();
            let database_url = config
                .get_table("databases")
                .expect(missing_database_error)
                .get("main_database")
                .expect(missing_database_error)
                .get("url")
                .expect(missing_database_error);
            database_url
                .as_str()
                .expect(missing_database_error)
                .to_owned()
        }
    };

    let conn = <ConnectionType as Connection>::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    embed_migrations!();

    embedded_migrations::run(&conn).expect("Failed running migrations");

    test_body(conn)
}

pub fn create_random_user(conn: &ConnectionType) -> User {
    let mut rng = thread_rng();
    let random_user_id = rng.gen_range(10000, i32::MAX);
    let username = format!("daniel-{}", random_user_id);

    let password: String = rng
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .collect();

    let user = User {
        id: random_user_id,
        username: username.clone(),
        first_name: "Daniel".to_string(),
        last_name: "Corn".to_string(),
        password: password.clone(),
    };

    UserRepository::new().add(conn, user.clone()).unwrap();

    user
}

pub fn create_setlist(conn: &ConnectionType, id: i32, user: i32) -> UserSetlist {
    let setlist = UserSetlist {
        id,
        user,
        user_name: "Saul".to_string(),
        sorting: id,
        entries: vec![
            SetlistEntry::new("song-1", FileType::Chorddown, "Song 1", None),
            SetlistEntry::new("song-2", FileType::Chorddown, "Song 2", None),
            SetlistEntry::new("song-3", FileType::Chorddown, "Song 3", None),
        ],
    };
    CommandExecutor::perform(&setlist, Command::add(conn)).unwrap();

    setlist
}
