pub mod command;
pub mod repository;

use crate::authentication::verify_password;
use crate::domain::credentials::Credentials;
use crate::domain::user::repository::UserRepository;
use crate::error::{AuthorizationError, SrvError};
use crate::schema::user;
use crate::traits::RecordIdTrait;
use crate::DbConn;
use diesel::Identifiable;
use libchordr::prelude::{Password, Username, RecordIdTrait};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use std::convert::TryInto;

#[table_name = "user"]
#[primary_key(username)]
#[derive(Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Debug, Clone)]
pub struct UserDb {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

impl UserDb {
    pub fn try_to_user(&self) -> Result<User, SrvError> {
        Ok(User::new(
            Username::new(self.username.clone())?,
            self.first_name.clone(),
            self.last_name.clone(),
            Password::hashed(self.password_hash.clone())?,
        ))
    }
}

impl TryInto<User> for UserDb {
    type Error = SrvError;

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User::new(
            Username::new(self.username)?,
            self.first_name,
            self.last_name,
            Password::hashed(self.password_hash)?,
        ))
    }
}

impl RecordIdTrait for UserDb {
    type Id = String;

    fn id(self) -> Self::Id {
        self.username
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserDb {
    type Error = AuthorizationError;

    /// Try to load the `User` from the Basic Auth header sent with `request`
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let db_outcome: Outcome<DbConn, _> = DbConn::from_request(request);
        match db_outcome {
            Outcome::Failure(_) => {
                Outcome::Failure((Status::Unauthorized, AuthorizationError::IncorrectUsername))
            }
            Outcome::Forward(val) => Outcome::Forward(val),
            Outcome::Success(conn) => {
                let authorization_headers: Vec<_> =
                    request.headers().get("Authorization").collect();

                let credentials = match Credentials::from_headers(authorization_headers.clone()) {
                    None => {
                        warn!("No credentials: {:?}", authorization_headers);
                        return Outcome::Failure((
                            Status::Unauthorized,
                            AuthorizationError::MissingCredentials,
                        ));
                    }
                    Some(c) => c,
                };

                match UserRepository::new().find_by_name(&conn.0, &credentials.username) {
                    Ok(user) if verify_password(&credentials, &user) => Outcome::Success(user),
                    Ok(_) => {
                        warn!("Wrong password");
                        Outcome::Failure((
                            Status::Unauthorized,
                            AuthorizationError::IncorrectPassword,
                        ))
                    }
                    Err(_e) => Outcome::Failure((
                        Status::Unauthorized,
                        AuthorizationError::IncorrectUsername,
                    )),
                }
            }
        }
    }
}
