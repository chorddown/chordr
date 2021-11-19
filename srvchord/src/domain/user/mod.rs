pub mod command;
pub mod repository;

use crate::authentication::verify_password;
use crate::domain::user::repository::UserRepository;
use crate::error::{AuthorizationError, SrvError};
use crate::schema::user;
use crate::traits::{FromHeader, FromHeaderResult};
use crate::DbConn;
use diesel::Identifiable;
use libchordr::prelude::{Credentials, Password, RecordTrait, User, Username};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use std::convert::TryInto;

#[derive(
    Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Debug, Clone,
)]
#[primary_key(username)]
#[table_name = "user"]
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

impl RecordTrait for UserDb {
    type Id = String;

    fn id(self) -> Self::Id {
        self.username
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserDb {
    type Error = AuthorizationError;

    /// Try to load the `User` from the Basic Auth header sent with `request`
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db_outcome: Outcome<DbConn, _> = DbConn::from_request(request).await;
        match db_outcome {
            Outcome::Failure(_) => {
                Outcome::Failure((Status::Unauthorized, AuthorizationError::IncorrectUsername))
            }
            Outcome::Forward(val) => Outcome::Forward(val),
            Outcome::Success(conn) => {
                let authorization_headers: Vec<_> =
                    request.headers().get("Authorization").collect();

                let credentials = match Credentials::from_headers(authorization_headers.clone()) {
                    FromHeaderResult::Ok(c) => c,
                    FromHeaderResult::None => {
                        warn!("No credentials: {:?}", authorization_headers);
                        return Outcome::Failure((
                            Status::Unauthorized,
                            AuthorizationError::MissingCredentials,
                        ));
                    }
                    FromHeaderResult::Err(e) => {
                        warn!(
                            "Could not decode credentials '{:?}': {}",
                            authorization_headers, e
                        );
                        return Outcome::Failure((
                            Status::Unauthorized,
                            AuthorizationError::MissingCredentials,
                        ));
                    }
                };

                conn.run(move |conn| {
                    match UserRepository::new().find_by_name(conn, &credentials.username()) {
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
                })
                .await
            }
        }
    }
}

impl FromHeader for Credentials {
    type Err = AuthorizationError;

    /// Try to read the `Credentials` from the Basic Auth header
    fn from_header(header: &str) -> FromHeaderResult<Self, Self::Err> {
        if !header.starts_with("Basic ") {
            return FromHeaderResult::None;
        }

        println!("{}", header);

        let header_chars = header.chars();
        let base64code = header_chars.into_iter().skip(6).collect::<String>();
        let decoded: String = match base64::decode(&base64code) {
            Ok(vec) => String::from_utf8_lossy(&vec).to_string(),
            Err(_) => return FromHeaderResult::None,
        };

        let parts: Vec<&str> = decoded.splitn(2, ':').collect();
        if parts.len() < 2 {
            return FromHeaderResult::None;
        }

        let username: Username = match parts[0].try_into() {
            Ok(u) => u,
            Err(_) => return FromHeaderResult::Err(AuthorizationError::MissingCredentials),
        };
        let password: Password = match parts[1].try_into() {
            Ok(p) => p,
            Err(_) => return FromHeaderResult::Err(AuthorizationError::MissingCredentials),
        };

        FromHeaderResult::Ok(Credentials::new(username, password))
    }
}
