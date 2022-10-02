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
pub struct SignUp {
    #[validate(length(min = 3, max = 15))]
    pub name: String,
    #[validate(regex = "PASSWORD_REGEX")]
    pub password: String,
}

#[derive(new, Debug)]
pub struct SignIn {
    pub name: String,
    pub password: String,
}

// NOTE: Domain
impl User {
    pub fn create(name: String, password: String) -> (Self, Tokens) {
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
        (user, tokens)
    }

    pub fn verify_password(&self, password: String) -> anyhow::Result<()> {
        verify(&password, &self.password_hash)
    }

    pub fn verify_refresh_token(&self, refresh_token: String) -> anyhow::Result<()> {
        let refresh_token_hash = self
            .refresh_token_hash
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("refresh_token_hash has already been empty"))?;
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

// NOTE: Commands
impl User {
    pub async fn sign_up(conn: &PgPool, input: SignUp) -> anyhow::Result<Tokens> {
        input.validate()?;
        let (user, tokens) = User::create(input.name, input.password);
        user.upsert(conn).await?;
        Ok(tokens)
    }

    pub async fn sign_in(conn: &PgPool, input: SignIn) -> anyhow::Result<Tokens> {
        let mut user = User::find_by_name(conn, input.name).await?;
        user.verify_password(input.password)?;
        let tokens = user.refresh_tokens();
        user.upsert(conn).await?;
        Ok(tokens)
    }

    pub async fn sign_out(conn: &PgPool, auth: Auth) -> anyhow::Result<()> {
        let mut user = User::find_by_id(conn, auth.sub).await?;
        user.unset_tokens();
        user.upsert(conn).await
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
