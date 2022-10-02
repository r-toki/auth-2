use super::lib::{
    date_time::get_current_date_time,
    id::get_new_id,
    jwt::{generate_tokens, Auth, Tokens},
    password::{hash, verify, PASSWORD_REGEX},
};
use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
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

// NOTE: Domain
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

// NOTE: SQL for Commands
impl User {
    pub async fn find_by_id(conn: &PgPool, id: String) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
select * from users
where id = $1
            "#,
            id
        )
        .fetch_one(conn)
        .await
        .map_err(Into::into)
    }

    pub async fn find_by_name(conn: &PgPool, name: String) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
select * from users
where name = $1
        "#,
            name
        )
        .fetch_one(conn)
        .await
        .map_err(Into::into)
    }

    pub async fn upsert(&self, conn: &PgPool) -> anyhow::Result<()> {
        query!(
            r#"
insert into users (id, name, password_hash, refresh_token_hash, created_at, updated_at)
values ($1, $2, $3, $4, $5, $6)
on conflict (id)
do update
set name = $2, password_hash = $3, refresh_token_hash = $4, created_at = $5, updated_at = $6
            "#,
            self.id,
            self.name,
            self.password_hash,
            self.refresh_token_hash,
            self.created_at,
            self.updated_at
        )
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
}
