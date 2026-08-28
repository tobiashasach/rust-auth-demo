use std::{error::Error, sync::LazyLock};

use chrono::{DateTime, TimeDelta, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::keys::RSA_PRIVATE_KEY;

static JWT_ENCODING_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
    let der_bytes = RSA_PRIVATE_KEY
        .private_key_to_der()
        .expect("Failed to export DER");

    EncodingKey::from_rsa_der(&der_bytes)
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<'a> {
    sub: &'a str,
    exp: i64,
}

pub fn jwt_sign(sub: &str) -> Result<(String, DateTime<Utc>), Box<dyn Error>> {
    let exp = Utc::now() + TimeDelta::days(7);

    let claims = Claims {
        sub,
        exp: exp.timestamp(),
    };

    let encoded_jwt = encode(&Header::new(Algorithm::RS256), &claims, &JWT_ENCODING_KEY)?;

    Ok((encoded_jwt, exp))
}
