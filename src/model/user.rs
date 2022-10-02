use super::lib::{
    id::get_new_id,
    password::{hash, PASSWORD_REGEX},
};
use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use validator::Validate;

#[derive(new, Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    password_hash: String,
    refresh_token_hash: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(new, Debug, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 15))]
    name: String,
    #[validate(regex = "PASSWORD_REGEX")]
    password: String,
}

impl User {
    pub async fn create(conn: &PgPool, input: CreateUser) -> anyhow::Result<()> {
        input.validate()?;
        query!(
            r#"
insert into users (id, name, password_hash, refresh_token_hash, created_at, updated_at)
values ($1, $2, $3, $4, current_timestamp, current_timestamp)
            "#,
            get_new_id(),
            input.name,
            hash(&input.password),
            None as Option<String>,
        )
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
    }
}
