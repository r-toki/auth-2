mod query;
mod repository;

pub use query::UserDto;

use super::lib::{date_time::get_current_date_time, id::get_new_id};
use crate::lib::{
    jwt::{generate_tokens, Auth, Tokens},
    password_hashing::{hash, verify},
};

use chrono::{DateTime, Utc};
use derive_new::new;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref RE_NAME: Regex = Regex::new(r"[A-Za-z\d#$@!%&*?]{3,15}").unwrap();
    static ref RE_PASSWORD: Regex = Regex::new(r"[A-Za-z\d#$@!%&*?]{8,30}").unwrap();
}

#[derive(new, Debug, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: String,
    #[validate(regex(
        path = "RE_NAME",
        message = "name must be at least 3 and no more than 15 characters in alphabet, numbers or symbols"
    ))]
    pub name: String,
    pub password_hash: String,
    pub refresh_token_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn create(name: String, password: String) -> anyhow::Result<Self> {
        if !RE_PASSWORD.is_match(&password) {
            anyhow::bail!("password must be at least 8 and no more than 30 characters in alphabet, numbers or symbols")
        }

        let id = get_new_id();
        let now = get_current_date_time();

        let user = User::new(id, name, hash(&password), None, now, now);
        user.validate()?;

        Ok(user)
    }

    pub fn issue_tokens(&mut self) -> Tokens {
        let tokens = generate_tokens(Auth::new(self.id.clone()));

        self.refresh_token_hash = Some(hash(&tokens.refresh_token));
        self.updated_at = get_current_date_time();

        tokens
    }

    pub fn revoke_tokens(&mut self) {
        self.refresh_token_hash = None;
        self.updated_at = get_current_date_time();
    }

    pub fn verify_password(&self, password: String) -> anyhow::Result<()> {
        verify(&password, &self.password_hash)
    }

    pub fn verify_refresh_token(&self, refresh_token: String) -> anyhow::Result<()> {
        let refresh_token_hash = self
            .refresh_token_hash
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("refresh_token_hash is empty"))?;

        verify(&refresh_token, refresh_token_hash)
    }
}
