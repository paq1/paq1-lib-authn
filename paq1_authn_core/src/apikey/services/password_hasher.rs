use paq1_lib_error_handler::prelude::ResultErr;

pub trait PasswordHasher {
    fn generate_pure_random(&self) -> ResultErr<String>;
    fn hashed(&self, pure: &str) -> ResultErr<String>;
}
