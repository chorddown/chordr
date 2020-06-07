use crate::domain::setlist::repository::UserSetlistRepository;
use crate::domain::setlist::UserSetlist;
use crate::domain::user::User;
use crate::traits::RepositoryTrait;
use crate::DbConn;
use rocket::{get, post};
use rocket_contrib::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        crate::routes::setlist::setlist_index,
        crate::routes::setlist::setlist_list,
        crate::routes::setlist::setlist_get,
        crate::routes::setlist::setlist_put,
        crate::routes::setlist::setlist_delete
    ]
}

#[get("/")]
pub fn setlist_index(conn: DbConn) -> Json<Vec<UserSetlist>> {
    Json(UserSetlistRepository::new().find_all(&conn.0).unwrap())
}

#[get("/<username>")]
pub fn setlist_list(username: String, conn: DbConn, user: User) -> Option<Json<Vec<UserSetlist>>> {
    println!("{:?}", user);

    match UserSetlistRepository::new().find_by_user(&conn.0, &user) {
        Ok(setlist) => Some(Json(setlist)),
        Err(_) => {
            warn!("No setlists for user {} found", username);
            None
        }
    }
}

#[get("/<username>/<setlist>")]
pub fn setlist_get(
    username: String,
    setlist: i32,
    conn: DbConn,
    user: User,
) -> Option<Json<UserSetlist>> {
    println!("{:?}", user);

    match UserSetlistRepository::new().find_by_user_and_setlist_id(&conn.0, &user, setlist) {
        Ok(setlist) => Some(Json(setlist)),
        Err(_) => {
            warn!("No setlists for user {} found", username);
            None
        }
    }
}

#[delete("/<username>/<setlist>")]
pub fn setlist_delete(
    username: String,
    setlist: i32,
    conn: DbConn,
    user: User,
) -> Option<()> {
    println!("{:?}", user);

    let repo = UserSetlistRepository::new();
    match repo.find_by_user_and_setlist_id(&conn.0, &user, setlist) {
        Ok(setlist) => {
            repo.delete(&conn.0, setlist).ok()
        }
        Err(_) => {
            warn!("No setlists for user {} found", username);
            None
        }
    }
}

#[post("/<username>", format = "application/json", data = "<setlist>")]
pub fn setlist_put(
    username: String,
    conn: DbConn,
    setlist: Json<UserSetlist>,
    user: User,
) -> Option<Json<UserSetlist>> {
    println!("Add/update setlist {} {:?}", username, setlist);
    let setlist = setlist.into_inner();

    let repo = UserSetlistRepository::new();
    match repo.find_by_user_and_setlist_id(&conn.0, &user, setlist.id) {
        Ok(_) => {
            println!("Perform update setlist {} #{}", username, setlist.id);
            match repo.update(&conn.0, setlist.clone()) {
                Ok(_) => Some(Json(setlist)),
                Err(e) => {
                    error!("{}", e);
                    None
                }
            }
        }
        Err(_) => {
            println!("Perform add setlist {} #{}", username, setlist.id);
            match repo.add(&conn.0, setlist.clone()) {
                Ok(_) => Some(Json(setlist)),
                Err(e) => {
                    error!("{}", e);
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test_helpers::{run_test_fn, json_format, JsonTemplateValue, create_random_user, create_setlist};
    use rocket::http::ContentType;
    use crate::traits::RepositoryTrait;
    use crate::domain::setlist::repository::UserSetlistRepository;
    use rand::Rng;
    use rocket::http::Status;
    use rocket::http::Header;

    #[test]
    fn test_get() {
        run_test_fn(|client, conn| {
            let user = create_random_user(&conn.0);
            let username = user.username;
            let password = user.password;
            let user_id = user.id;

            let mut rng = rand::thread_rng();
            let random_id = rng.gen_range(10000, i32::MAX);
            create_setlist(&conn.0, random_id, user_id);

            // Issue a request to insert a new setlist
            let encoded_credentials = base64::encode(format!("{}:{}", username, password));
            let authorization_header = Header::new("Authorization", format!("Basic {}", encoded_credentials));

            let mut get_response = client
                .get(format!("/setlist/{}/{}", username, random_id))
                .header(authorization_header.clone())
                .dispatch();
            assert_eq!(
                get_response.status(),
                Status::Ok
            );

            let response_body = get_response.body_string().unwrap();

            assert_eq!(
                response_body,
                json_format::<JsonTemplateValue>(
                    r#"{"id":$,"user":$,"user_name":"Saul","sorting":$,"entries":[{"song_id":"song-1","file_type":"chorddown","title":"Song 1"},{"song_id":"song-2","file_type":"chorddown","title":"Song 2"},{"song_id":"song-3","file_type":"chorddown","title":"Song 3"}]}"#,
                    vec![
                        random_id.into(),
                        user_id.into(),
                        random_id.into(),
                    ],
                )
            );
        })
    }

    #[test]
    fn test_insertion_deletion() {
        run_test_fn(|client, conn| {
            let mut rng = rand::thread_rng();
            let user_setlist_repository = UserSetlistRepository::new();
            let initial_count = user_setlist_repository.count_all(&conn.0).unwrap();

            let user = create_random_user(&conn.0);
            let username = user.username;
            let password = user.password;
            let user_id = user.id;

            let random_id = rng.gen_range(10000, i32::MAX);

            // Issue a request to insert a new setlist
            let encoded_credentials = base64::encode(format!("{}:{}", username, password));
            let authorization_header = Header::new("Authorization", format!("Basic {}", encoded_credentials));
            let post_response = client
                .post(format!("/setlist/{}", username))
                .header(ContentType::JSON)
                .header(authorization_header.clone())
                .body(
                    json_format::<JsonTemplateValue>(
                        r#"{"id":$,"user":$,"user_name":"$","sorting":200,"entries":[{"song_id":"great","file_type":"chorddown","title":"song 2"},{"song_id":"tune","file_type":"chorddown","title":"song 1"}]}"#,
                        vec![
                            random_id.into(),
                            user_id.into(),
                            username.clone().into()
                        ],
                    )
                )
                .dispatch();
            assert_eq!(post_response.status(), Status::Ok);

            // Ensure we have one more setlist in the database
            assert_eq!(user_setlist_repository.count_all(&conn.0).unwrap(), initial_count + 1);

            assert_eq!(
                client
                    .get(format!("/setlist/{}/{}", username, random_id))
                    .header(authorization_header.clone())
                    .dispatch().status(),
                Status::Ok
            );

            // Ensure the setlist is what we expect
            let setlist = user_setlist_repository.find_by_id(&conn.0, random_id).expect("New setlist not found");
            assert_eq!(setlist.entries.len(), 2);
            assert_eq!(setlist.user_name, username);

            // Issue a request to delete the setlist
            let delete_response = client
                .delete(format!("/setlist/{}/{}", username, random_id))
                .header(authorization_header.clone())
                .dispatch();
            println!("{}", delete_response.status());
            assert_eq!(delete_response.status(), Status::Ok);

            // Ensure it's gone
            assert_eq!(user_setlist_repository.count_all(&conn.0).unwrap(), initial_count);
            assert_eq!(
                client
                    .get(format!("/setlist/{}/{}", username, random_id))
                    .header(authorization_header.clone())
                    .dispatch().status(),
                Status::NotFound
            );
        })
    }

    #[test]
    fn test_update() {
        run_test_fn(|client, conn| {
            let mut rng = rand::thread_rng();
            let user_setlist_repository = UserSetlistRepository::new();

            let user = create_random_user(&conn.0);
            let username = user.username;
            let password = user.password;
            let user_id = user.id;

            let random_id = rng.gen_range(10000, i32::MAX);
            create_setlist(&conn.0, random_id, user_id);
            let initial_count = user_setlist_repository.count_all(&conn.0).unwrap();

            // Issue a request to insert a new setlist
            let encoded_credentials = base64::encode(format!("{}:{}", username, password));
            let authorization_header = Header::new("Authorization", format!("Basic {}", encoded_credentials));
            let post_response = client
                .post(format!("/setlist/{}", username))
                .header(ContentType::JSON)
                .header(authorization_header.clone())
                .body(
                    json_format::<JsonTemplateValue>(
                        r#"{"id":$,"user":$,"user_name":"$","sorting":200,"entries":[]}"#,
                        vec![
                            random_id.into(),
                            user_id.into(),
                            "Daniel".into()
                        ],
                    )
                )
                .dispatch();
            assert_eq!(post_response.status(), Status::Ok);

            // Ensure we have the same number of setlists in the database
            assert_eq!(user_setlist_repository.count_all(&conn.0).unwrap(), initial_count);

            // Ensure the setlist is what we expect
            let setlist = user_setlist_repository.find_by_id(&conn.0, random_id).expect("New setlist not found");
            assert_eq!(setlist.entries.len(), 0);
            assert_eq!(&setlist.user_name, "Daniel");
        })
    }
}
