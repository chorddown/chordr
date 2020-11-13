use crate::domain::user::UserDb;
use libchordr::prelude::User;
use rocket::get;
use rocket_contrib::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        crate::routes::user::index,
    ]
}

#[get("/")]
pub fn index(user: UserDb) -> Option<Json<User>> {
    match user.try_to_user() {
        Ok(u) => Some(Json(u)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod test {
    use crate::test_helpers::{
        create_random_user, json_format, run_test_fn, JsonTemplateValue,
    };
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

            let mut get_response = client
                .get("/user/")
                .header(authorization_header.clone())
                .dispatch();
            assert_eq!(get_response.status(), Status::Ok);

            let response_body = get_response.body_string().unwrap();

            assert_eq!(
                response_body,
                json_format::<JsonTemplateValue>(
                    r#"{"username":"$","first_name":"Daniel","last_name":"Corn"}"#,
                    vec![
                        username.into(),
                    ],
                )
            );
        })
    }
}
