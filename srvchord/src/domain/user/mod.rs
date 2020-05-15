pub mod command;
pub mod repository;

use crate::authentication::verify_password;
use crate::domain::credentials::Credentials;
use crate::domain::user::repository::UserRepository;
use crate::schema::user;
use crate::traits::RecordIdTrait;
use crate::DbConn;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use crate::error::AuthorizationError;

#[table_name = "user"]
#[derive(Serialize, Deserialize, Identifiable, Queryable, Insertable, AsChangeset, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl RecordIdTrait for User {
    type Id = i32;

    fn id(self) -> Self::Id {
        self.id
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = AuthorizationError;

    /// Try to load the `User` from the Basic Auth header sent with `request`
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let db_outcome: Outcome<DbConn, _> = DbConn::from_request(request);
        match db_outcome {
            Outcome::Failure(_) => Outcome::Failure((Status::Unauthorized, AuthorizationError::IncorrectUsername)),
            Outcome::Forward(val) => Outcome::Forward(val),
            Outcome::Success(conn) => {
                let authorization_headers: Vec<_> = request.headers().get("Authorization").collect();

                let credentials = match Credentials::from_headers(authorization_headers.clone()) {
                    None => {
                        warn!("No credentials: {:?}", authorization_headers);
                        return Outcome::Failure((Status::Unauthorized, AuthorizationError::MissingCredentials));
                    }
                    Some(c) => c,
                };

                match UserRepository::new().find_by_name(&conn.0, &credentials.username) {
                    Ok(user) if verify_password(&credentials, &user) => Outcome::Success(user),
                    Ok(_) => {
                        warn!("Wrong password");
                        Outcome::Failure((Status::Unauthorized, AuthorizationError::IncorrectPassword))
                    }
                    Err(_e) => Outcome::Failure((Status::Unauthorized, AuthorizationError::IncorrectUsername)),
                }
            }
        }
    }
}
