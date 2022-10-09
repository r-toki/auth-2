use super::User;

use sqlx::{query, query_as, PgExecutor};

impl User {
    pub async fn find_by_id(
        executor: impl PgExecutor<'_>,
        id: String,
    ) -> anyhow::Result<Option<User>> {
        query_as!(
            User,
            r#"
select * from users
where id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }

    pub async fn find_by_name(
        executor: impl PgExecutor<'_>,
        name: String,
    ) -> anyhow::Result<Option<User>> {
        query_as!(
            User,
            r#"
select * from users
where name = $1
        "#,
            name
        )
        .fetch_optional(executor)
        .await
        .map_err(Into::into)
    }

    pub async fn upsert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
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
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
}
