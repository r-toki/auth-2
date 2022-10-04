use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify(password: &str, password_hash: &str) -> anyhow::Result<()> {
    let password_hash = PasswordHash::new(&password_hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|e| anyhow::anyhow!("{}", e))
}
