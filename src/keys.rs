use openssl::pkey::{PKey, Private, Public};
use std::{fs::read_to_string, sync::LazyLock};

pub static RSA_PRIVATE_KEY: LazyLock<PKey<Private>> = LazyLock::new(|| {
    let pem =
        read_to_string("./keys/private_key.pem").expect("Failed to read ./keys/private_key.pem");

    PKey::private_key_from_pem(pem.as_bytes()).expect("Failed to parse private key PEM")
});

pub static RSA_PUBLIC_KEY: LazyLock<PKey<Public>> = LazyLock::new(|| {
    let pem =
        read_to_string("./keys/public_key.pem").expect("Failed to read ./keys/public_key.pem");

    PKey::public_key_from_pem(pem.as_bytes()).expect("Failed to parse public key PEM")
});
