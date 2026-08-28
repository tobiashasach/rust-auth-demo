use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use std::error::Error;
use zeroize::Zeroizing;

use crate::config::CONFIG;

pub fn password_hash(password: Zeroizing<String>) -> Result<String, Box<dyn Error>> {
    let argon2 = Argon2::new_with_secret(
        CONFIG.pepper().as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )?;

    Ok(argon2.hash_password(password.as_bytes())?.to_string())
}

pub fn password_verify(
    password: Zeroizing<String>,
    password_hash: &str,
) -> Result<(), Box<dyn Error>> {
    let parsed_hash = PasswordHash::new(&password_hash)?;

    let argon2 = Argon2::new_with_secret(
        CONFIG.pepper().as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )?;

    argon2.verify_password(password.as_bytes(), &parsed_hash)?;

    Ok(())
}
