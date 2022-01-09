use rocket::serde::json::Json;
use rocket::{get, post};

use cqrs::prelude::RepositoryTrait as CqrsRepositoryTrait;
use libchordr::prelude::{Setlist, Username};

use crate::domain::setlist::repository::SetlistRepository;
use crate::domain::user::UserDb;
use crate::DbConn;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        crate::routes::setlist::setlist_index,
        crate::routes::setlist::setlist_list,
        crate::routes::setlist::setlist_get,
        crate::routes::setlist::setlist_get_latest,
        crate::routes::setlist::setlist_put,
        crate::routes::setlist::setlist_delete
    ]
}

#[get("/")]
pub async fn setlist_index(conn: DbConn) -> Json<Vec<Setlist>> {
    conn.run(move |conn| Json(SetlistRepository::new().find_all(conn).unwrap()))
        .await
}

#[get("/<username>")]
pub async fn setlist_list(
    username: String,
    conn: DbConn,
    user: UserDb,
) -> Option<Json<Vec<Setlist>>> {
    let username_instance = match check_username(&username, &user) {
        Ok(u) => u,
        Err(_) => return None,
    };

    conn.run(move |conn| {
        match SetlistRepository::new().find_by_username(conn, &username_instance) {
            Ok(setlists) => Some(Json(setlists)),
            Err(e) => {
                warn!("No setlists for user {} found: {}", username, e);
                None
            }
        }
    })
    .await
}

#[get("/<username>/<setlist>")]
pub async fn setlist_get(
    username: String,
    setlist: i32,
    conn: DbConn,
    user: UserDb,
) -> Option<Json<Setlist>> {
    let username_instance = match check_username(&username, &user) {
        Ok(u) => u,
        Err(_) => return None,
    };

    conn.run(move |conn| {
        match SetlistRepository::new().find_by_username_and_setlist_id(
            conn,
            &username_instance,
            setlist,
        ) {
            Ok(setlist) => Some(Json(setlist)),
            Err(_) => {
                warn!("Setlist {} for user {} not found", setlist, username);
                None
            }
        }
    })
    .await
}

#[get("/<username>/latest", rank = 2)]
pub async fn setlist_get_latest(
    username: String,
    conn: DbConn,
    user: UserDb,
) -> Option<Json<Setlist>> {
    let username_instance = match check_username(&username, &user) {
        Ok(u) => u,
        Err(_) => return None,
    };

    conn.run(move |conn| {
        match SetlistRepository::new().find_by_username(conn, &username_instance) {
            Ok(setlists) if setlists.is_empty() => {
                warn!("No setlists for user {} found", username);
                None
            }
            Ok(mut setlists) => {
                setlists.sort_by(|a, b| {
                    a.modification_date()
                        .partial_cmp(&b.modification_date())
                        .unwrap()
                });

                Some(Json(setlists.pop().unwrap()))
            }
            Err(e) => {
                warn!("No setlists for user {} found: {}", username, e);
                None
            }
        }
    })
    .await
}

#[delete("/<username>/<setlist>")]
pub async fn setlist_delete(
    username: String,
    setlist: i32,
    conn: DbConn,
    user: UserDb,
) -> Option<()> {
    let username_instance = match check_username(&username, &user) {
        Ok(u) => u,
        Err(_) => return None,
    };

    let repo = SetlistRepository::new();
    conn.run(move |conn| {
        match repo.find_by_username_and_setlist_id(conn, &username_instance, setlist) {
            Ok(setlist) => repo.delete(conn, setlist).ok(),
            Err(_) => {
                warn!("Setlist {} for user {} not found", setlist, username);
                None
            }
        }
    })
    .await
}

#[post("/<username>", format = "application/json", data = "<setlist>")]
pub async fn setlist_put(
    username: String,
    conn: DbConn,
    setlist: Json<Setlist>,
    user: UserDb,
) -> Option<Json<Setlist>> {
    let username_instance = match check_username(&username, &user) {
        Ok(u) => u,
        Err(_) => return None,
    };
    debug!("Add/update setlist {} {:?}", username, setlist);

    // Todo: Check if user did not change
    let setlist = setlist.into_inner();

    let repo = SetlistRepository::new();
    conn.run(move |conn| {
        match repo.find_by_username_and_setlist_id(conn, &username_instance, setlist.id()) {
            Ok(_) => {
                info!("Perform update setlist {} #{}", username, setlist.id());
                match repo.update(conn, setlist.clone()) {
                    Ok(_) => Some(Json(setlist)),
                    Err(e) => {
                        error!("{}", e);
                        None
                    }
                }
            }
            Err(_) => {
                info!("Perform add setlist {} #{}", username, setlist.id());
                match repo.add(conn, setlist.clone()) {
                    Ok(_) => Some(Json(setlist)),
                    Err(e) => {
                        error!("{}", e);
                        None
                    }
                }
            }
        }
    })
    .await
}

fn check_username(username: &str, user: &UserDb) -> Result<Username, ()> {
    if user.username != username {
        warn!(
            "Logged in user {} has no access to setlists of user {}",
            user.username, username
        );

        Err(())
    } else {
        match Username::new(username) {
            Ok(u) => Ok(u),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use rand::Rng;
    use rocket::http::ContentType;
    use rocket::http::Header;
    use rocket::http::Status;

    use libchordr::prelude::ListTrait;

    use crate::domain::setlist::repository::SetlistRepository;
    use crate::test_helpers::{
        create_random_user, create_setlist, json_format, run_test_fn, JsonTemplateValue,
    };
    // use crate::traits::RepositoryTrait;
    use cqrs::prelude::RepositoryTrait;

    #[test]
    fn test_get() {
        run_test_fn(|client, conn| {
            let user = create_random_user(&conn.0);
            let username = user.username;
            let password = user.password_hash;

            let mut rng = rand::thread_rng();
            let random_id = rng.gen_range(10000, i32::MAX);
            let setlist = create_setlist(&conn.0, random_id, username.clone());

            // Issue a request to insert a new setlist
            let encoded_credentials = base64::encode(format!("{}:{}", username, password));
            let authorization_header =
                Header::new("Authorization", format!("Basic {}", encoded_credentials));

            let get_response = client
                .get(format!("/setlist/{}/{}", username, random_id))
                .header(authorization_header.clone())
                .dispatch();
            assert_eq!(get_response.status(), Status::Ok);

            let response_body = get_response.into_string().unwrap();

            assert_eq!(
                response_body,
                json_format::<JsonTemplateValue>(
                    r#"{"name":"My setlist","id":$,"owner":{"username":"$","first_name":"Daniel","last_name":"Corn"},"team":null,"songs":$,"gig_date":null,"creation_date":"$","modification_date":"$"}"#,
                    vec![
                        random_id.into(),
                        username.into(),
                        r#"[{"song_id":"song-1","file_type":"chorddown","title":"Song 1","settings":null},{"song_id":"song-2","file_type":"chorddown","title":"Song 2","settings":null},{"song_id":"song-3","file_type":"chorddown","title":"Song 3","settings":null}]"#.into(),
                        format!("{:?}", setlist.creation_date()).into(),
                        format!("{:?}", setlist.modification_date()).into(),
                    ],
                )
            );
        })
    }

    #[test]
    fn test_insertion_deletion() {
        run_test_fn(|client, conn| {
            let mut rng = rand::thread_rng();
            let user_setlist_repository = SetlistRepository::new();
            let initial_count = user_setlist_repository.count_all(&conn.0).unwrap();

            let user = create_random_user(&conn.0);
            let username = user.username;
            let password = user.password_hash;

            let random_id = rng.gen_range(10000, i32::MAX);
            let now = Utc::now();

            // Issue a request to insert a new setlist
            let encoded_credentials = base64::encode(format!("{}:{}", username, password));
            let authorization_header =
                Header::new("Authorization", format!("Basic {}", encoded_credentials));
            let post_response = client
                .post(format!("/setlist/{}", username))
                .header(ContentType::JSON)
                .header(authorization_header.clone())
                .body(
                    json_format::<JsonTemplateValue>(
                        r#"{"name":"My setlist","id":$,"owner":{"username":"$","first_name":"Daniel","last_name":"Corn","password":"$"},"team":null,"songs":$,"gig_date":null,"creation_date":"$","modification_date":"$"}"#,
                        vec![
                            random_id.into(),
                            username.clone().into(),
                            password.into(),
                            r#"[{"song_id":"great","file_type":"chorddown","title":"song 2"},{"song_id":"tune","file_type":"chorddown","title":"song 1"}]"#.into(),
                            format!("{:?}", now).into(),
                            format!("{:?}", now).into(),
                        ],
                    )
                )
                .dispatch();
            assert_eq!(post_response.status(), Status::Ok);

            // Ensure we have one more setlist in the database
            assert_eq!(
                user_setlist_repository.count_all(&conn.0).unwrap(),
                initial_count + 1
            );

            assert_eq!(
                client
                    .get(format!("/setlist/{}/{}", username, random_id))
                    .header(authorization_header.clone())
                    .dispatch()
                    .status(),
                Status::Ok
            );

            // Ensure the setlist is what we expect
            let setlist = user_setlist_repository
                .find_by_id(&conn.0, random_id)
                .expect_some("New setlist not found");
            assert_eq!(setlist.len(), 2);
            assert_eq!(setlist.owner().username().to_string(), username);

            // Issue a request to delete the setlist
            let delete_response = client
                .delete(format!("/setlist/{}/{}", username, random_id))
                .header(authorization_header.clone())
                .dispatch();
            assert_eq!(delete_response.status(), Status::Ok);

            // Ensure it's gone
            assert_eq!(
                user_setlist_repository.count_all(&conn.0).unwrap(),
                initial_count
            );
            assert_eq!(
                client
                    .get(format!("/setlist/{}/{}", username, random_id))
                    .header(authorization_header.clone())
                    .dispatch()
                    .status(),
                Status::NotFound
            );
        })
    }

    #[test]
    fn test_update() {
        run_test_fn(|client, conn| {
            let mut rng = rand::thread_rng();
            let user_setlist_repository = SetlistRepository::new();

            let user = create_random_user(&conn.0);
            let username = user.username;
            let password = user.password_hash;

            let random_id = rng.gen_range(10000, i32::MAX);
            create_setlist(&conn.0, random_id, username.clone());
            let initial_count = user_setlist_repository.count_all(&conn.0).unwrap();
            let now = Utc::now();

            // Issue a request to insert a new setlist
            let encoded_credentials = base64::encode(format!("{}:{}", username, password));
            let authorization_header =
                Header::new("Authorization", format!("Basic {}", encoded_credentials));
            let post_response = client
                .post(format!("/setlist/{}", username))
                .header(ContentType::JSON)
                .header(authorization_header.clone())
                .body(
                    json_format::<JsonTemplateValue>(
                        r#"{"name":"My setlist","id":$,"owner":{"username":"$","first_name":"Daniel","last_name":"Corn","password":"$"},"team":null,"songs":$,"gig_date":null,"creation_date":"$","modification_date":"$"}"#,
                        vec![
                            random_id.into(),
                            username.clone().into(),
                            password.into(),
                            r#"[]"#.into(),
                            format!("{:?}", now).into(),
                            format!("{:?}", now).into(),
                        ],
                    )
                )
                .dispatch();
            assert_eq!(post_response.status(), Status::Ok);

            // Ensure we have the same number of setlists in the database
            assert_eq!(
                user_setlist_repository.count_all(&conn.0).unwrap(),
                initial_count
            );

            // Ensure the setlist is what we expect
            let setlist = user_setlist_repository
                .find_by_id(&conn.0, random_id)
                .expect_some("New setlist not found");
            assert_eq!(setlist.len(), 0);
            assert_eq!(setlist.owner().username().to_string().as_str(), username);
        })
    }
}
