use super::User;

use chrono::{DateTime, Utc};
use derive_new::new;
use serde::Serialize;
use sqlx::{query_as, PgExecutor};

#[derive(new, Debug, Serialize)]
pub struct UserDto {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_user(executer: impl PgExecutor<'_>, id: String) -> anyhow::Result<UserDto> {
        query_as!(
            UserDto,
            r#"
select id, name, created_at, updated_at from users
where id = $1
            "#,
            id
        )
        .fetch_one(executer)
        .await
        .map_err(Into::into)
    }
}
