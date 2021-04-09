mod json_formatting;

pub use self::json_formatting::*;
use crate::domain::setlist::command::SetlistCommandExecutor;
use crate::domain::user::command::UserCommandExecutor;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::UserDb;
use crate::traits::RepositoryTrait;
use crate::{ConnectionType, DbConn};
use chrono::Utc;
use cqrs::prelude::{Command, CommandExecutor};
use diesel::Connection;
use libchordr::models::user::User;
use libchordr::prelude::{FileType, Password, Setlist, SetlistEntry, Username};
use parking_lot::Mutex;
use rand::{thread_rng, Rng};
use rocket::config::RocketConfig;
use rocket::local::Client;

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

pub fn create_random_user(conn: &ConnectionType) -> UserDb {
    let mut rng = thread_rng();
    let random_user_id = rng.gen_range(10000, i32::MAX);
    let username = format!("daniel-{}", random_user_id);

    let password: String = rng
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .collect();

    let user = UserDb {
        username: username.clone(),
        first_name: "Daniel".to_string(),
        last_name: "Corn".to_string(),
        password_hash: password.clone(),
    };

    UserRepository::new().add(conn, user.clone()).unwrap();

    user
}

pub fn insert_test_user<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
    conn: &ConnectionType,
    username: S1,
    first_name: S2,
    last_name: S3,
) -> UserDb {
    let username = username.into();
    let first_name = first_name.into();
    let last_name = last_name.into();
    let new_user = UserDb {
        username,
        first_name,
        last_name,
        password_hash: create_test_password().to_string(),
    };

    CommandExecutor::perform(
        &UserCommandExecutor::with_connection(conn),
        Command::add(new_user.clone()),
    )
        .unwrap();

    new_user
}

pub fn create_setlist<S: AsRef<str>>(conn: &ConnectionType, id: i32, username: S) -> Setlist {
    let now = Utc::now();

    let setlist = Setlist::new(
        "My setlist",
        id,
        User::new(
            Username::new(username.as_ref()).unwrap(),
            "Saul",
            "Doe",
            create_test_password(),
        ),
        None,
        None,
        now,
        now,
        vec![
            SetlistEntry::new("song-1", FileType::Chorddown, "Song 1", None),
            SetlistEntry::new("song-2", FileType::Chorddown, "Song 2", None),
            SetlistEntry::new("song-3", FileType::Chorddown, "Song 3", None),
        ],
    );

    CommandExecutor::perform(
        &SetlistCommandExecutor::with_connection(&conn),
        Command::add(setlist.clone()),
    )
        .unwrap();

    setlist
}

pub fn create_test_password() -> Password {
    Password::new("a-super-nice-password").unwrap()
}
