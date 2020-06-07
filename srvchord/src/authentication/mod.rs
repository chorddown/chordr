use crate::domain::credentials::Credentials;
use crate::domain::user::User;
use crate::error::SrvError;

pub fn verify_password(credentials: &Credentials, user: &User) -> bool {
    let password_data = &user.password;
    let parts: Vec<&str> = password_data.splitn(3, ':').collect();
    if parts.len() < 3 {
        warn!("Check un-hashed password");

        return password_data == &credentials.password;
    }

    let algorithm = parts[0];
    let salt = parts[1];
    let hash = parts[2];
    if algorithm == "argon2" {
        match verify_password_argon2(&credentials.password, hash, salt) {
            Ok(r) => r,
            Err(e) => {
                error!("{}", e);
                false
            }
        }
    } else {
        error!("Unsupported password hashing algorithm: {}", algorithm);
        false
    }
}

fn verify_password_argon2(password: &str, hash: &str, salt: &str) -> Result<bool, SrvError> {
    use argon2::Config;

    #[cfg(debug_assertions)]
        let real_password_hash = {
        let config = Config::default();
        match argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config) {
            Ok(r) => Some(r),
            Err(e) => {
                error!("Could not generate hash for sent password: {}", e);
                None
            }
        }
    };

    match argon2::verify_encoded(&hash, password.as_bytes()) {
        Ok(v) if v == true => Ok(true),
        Ok(_) => {
            #[cfg(debug_assertions)]
            if let Some(real_password_hash) = real_password_hash {
                warn!(
                    "Password hash does not match. Hash of given password is: `{}`",
                    real_password_hash
                );
            }

            Ok(false)
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            if let Some(real_password_hash) = real_password_hash {
                warn!(
                    "Password hash from DB could not be decoded. Hash of given password is: `{}`",
                    real_password_hash
                );
            }

            Err(e.into())
        }
    }
}
