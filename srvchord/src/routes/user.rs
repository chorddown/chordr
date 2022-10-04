use crate::domain::setlist::repository::SetlistRepository;
use crate::domain::user::UserDb;
use crate::DbConn;
use libchordr::models::setlist::Setlist;
use libchordr::models::user::MainData;
use libchordr::prelude::User;
use rocket::get;
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        crate::routes::user::index,
        crate::routes::user::index_options,
        crate::routes::user::data,
    ]
}

#[get("/")]
pub fn index(user: UserDb) -> Option<Json<User>> {
    match user.try_to_user() {
        Ok(u) => Some(Json(u)),
        Err(_) => None,
    }
}

#[options("/")]
pub fn index_options() -> () {}

#[get("/data")]
pub async fn data(user_db: UserDb, conn: DbConn) -> Option<Json<MainData>> {
    conn.run(move |conn| match user_db.try_to_user() {
        Ok(user) => {
            let username = user.username();
            let latest_setlist = match SetlistRepository::new(conn).find_by_username(username) {
                Ok(setlists) if setlists.is_empty() => None,
                Ok(mut setlists) => {
                    sort_setlist_by_date(&mut setlists);

                    Some(setlists.pop().unwrap())
                }
                Err(e) => {
                    warn!("No setlists for user {} found: {}", username, e);
                    None
                }
            };

            Some(Json(MainData {
                user,
                latest_setlist,
                song_settings: None,
            }))
        }
        Err(_) => None,
    })
    .await
}

fn sort_setlist_by_date(setlists: &mut Vec<Setlist>) {
    setlists.sort_by(|a, b| {
        a.modification_date()
            .partial_cmp(&b.modification_date())
            .unwrap()
    });
}

#[cfg(test)]
mod test {
    use crate::test_helpers::{create_random_user, json_format, run_test_fn, JsonTemplateValue};
    use rocket::http::Header;
    use rocket::http::Status;

    #[test]
    fn test_index() {
        run_test_fn(|client, conn| {
            let user = create_random_user(&conn.0);
            let username = user.username;
            let password = user.password_hash;

            // Issue a request to insert a new setlist
            let encoded_credentials = base64::encode(format!("{}:{}", username, password));
            let authorization_header =
                Header::new("Authorization", format!("Basic {}", encoded_credentials));

            let get_response = client
                .get("/api/user/")
                .header(authorization_header.clone())
                .dispatch();
            assert_eq!(get_response.status(), Status::Ok);

            let response_body = get_response.into_string().unwrap();

            assert_eq!(
                response_body,
                json_format::<JsonTemplateValue>(
                    r#"{"username":"$","first_name":"Daniel","last_name":"Corn"}"#,
                    vec![username.into(),],
                )
            );
        })
    }
}
