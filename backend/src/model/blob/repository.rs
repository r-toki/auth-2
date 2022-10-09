use super::Blob;

use sqlx::{query, query_as, PgExecutor};

impl Blob {
    pub async fn find_by_id(executor: impl PgExecutor<'_>, id: String) -> anyhow::Result<Blob> {
        query_as!(
            Blob,
            r#"
select * from blobs
where id = $1
            "#,
            id
        )
        .fetch_one(executor)
        .await
        .map_err(Into::into)
    }

    pub async fn upsert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
insert into blobs (id, data, name, content_type, byte_size, created_at, updated_at)
values ($1, $2, $3, $4, $5, $6, $7)
on conflict (id)
do update
set data = $2, name = $3, content_type = $4, byte_size = $5, created_at = $6, updated_at = $7
            "#,
            self.id,
            self.data,
            self.name,
            self.content_type,
            self.byte_size,
            self.created_at,
            self.updated_at
        )
        .execute(executor)
        .await
        .map(|_| ())
        .map_err(Into::into)
    }
}
