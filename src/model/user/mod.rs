mod repository;

use super::lib::{date_time::get_current_date_time, id::get_new_id, password::PASSWORD_REGEX};
use crate::lib::{
    jwt::{generate_tokens, Auth, Tokens},
    password_hashing::{hash, verify},
};
use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(new, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password_hash: String,
    pub refresh_token_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(new, Debug, Validate)]
pub struct Create {
    #[validate(length(min = 3, max = 15))]
    pub name: String,
    #[validate(regex = "PASSWORD_REGEX")]
    pub password: String,
}

impl User {
    pub fn create(name: String, password: String) -> anyhow::Result<(Self, Tokens)> {
        Create::new(name.clone(), password.clone()).validate()?;

        let id = get_new_id();
        let tokens = generate_tokens(Auth::new(id.clone()));
        let now = get_current_date_time();

        let user = User::new(
            id,
            name,
            hash(&password),
            Some(hash(&tokens.refresh_token)),
            now,
            now,
        );

        Ok((user, tokens))
    }

    pub fn verify_password(&self, password: String) -> anyhow::Result<()> {
        verify(&password, &self.password_hash)
    }

    pub fn verify_refresh_token(&self, refresh_token: String) -> anyhow::Result<()> {
        let refresh_token_hash = self
            .refresh_token_hash
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("refresh_token_hash must not be empty"))?;

        verify(&refresh_token, refresh_token_hash)
    }

    pub fn refresh_tokens(&mut self) -> Tokens {
        let tokens = generate_tokens(Auth::new(self.id.clone()));

        self.refresh_token_hash = Some(hash(&tokens.refresh_token));
        self.updated_at = get_current_date_time();

        tokens
    }

    pub fn unset_tokens(&mut self) {
        self.refresh_token_hash = None;
        self.updated_at = get_current_date_time();
    }
}
