use std::str::FromStr;
use base64::{engine::general_purpose, Engine as _};
use paq1_authn_core::apikey::services::password_hasher::PasswordHasher;
use paq1_lib_error_handler::prelude::{Error, ErrorWithCode, ResultErr};
use password_hash::phc::SaltString;
use password_hash::PasswordHasher as PH;
use rand::rngs::OsRng;
use rand::TryRngCore;

pub struct DefaultPasswordHasher {
    pub salt: String,
}

impl DefaultPasswordHasher {
    pub fn new(salt: &str) -> Self {
        Self {
            salt: salt.to_string(),
        }
    }
}

impl PasswordHasher for DefaultPasswordHasher {
    fn generate_pure_random(&self) -> ResultErr<String> {
        let mut key_bytes = [0u8; 32];
        OsRng.try_fill_bytes(&mut key_bytes).map_err(|err| {
            Error::Failure(
                ErrorWithCode::new("FGPWERR", 500, "erreur de génération de mot de passe")
                    .with_description(format!("{err}").as_str()),
            )
        })?;
        let pwd_hash = general_purpose::STANDARD.encode(&key_bytes);
        Ok(pwd_hash)
    }

    fn hashed(&self, pure: &str) -> ResultErr<String> {
        let fixed_salt = SaltString::from_str(self.salt.as_str()).unwrap();
        let argon2 = argon2::Argon2::default();
        let pwd = argon2
            .hash_password_with_salt(pure.as_bytes(), fixed_salt.as_bytes())
            .map_err(|err| {
                Error::Failure(ErrorWithCode {
                    code: "HASHPW".to_string(),
                    status: 500,
                    title: "Erreur lors du hash du pwd (argon2)".to_string(),
                    description: Some(err.to_string()),
                    problems: vec![],
                })
            })?;
        pwd.hash
            .map(|underlying| underlying.to_string())
            .ok_or(Error::Failure(ErrorWithCode {
                code: "NFHASH".to_string(),
                status: 500,
                title: "Pas de hash trouver.".to_string(),
                description: None,
                problems: vec![],
            }))
    }
}
