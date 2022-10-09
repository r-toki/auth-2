use super::Attachment;

use sqlx::{query, query_as, PgExecutor};

impl Attachment {
    pub async fn find_by_record(
        executor: impl PgExecutor<'_>,
        record_type: String,
        record_id: String,
        record_name: String,
    ) -> anyhow::Result<Vec<Attachment>> {
        query_as!(
            Attachment,
            r#"
select * from attachments
where record_type = $1 and record_id = $2 and record_name = $3
            "#,
            record_type,
            record_id,
            record_name
        )
        .fetch_all(executor)
        .await
        .map_err(Into::into)
    }

    pub async fn upsert(&self, executor: impl PgExecutor<'_>) -> anyhow::Result<()> {
        query!(
            r#"
insert into attachments (id, record_type, record_id, record_name, blob_id, created_at, updated_at)
values ($1, $2, $3, $4, $5, $6, $7)
on conflict (id)
do update
set record_type = $2, record_id = $3, record_name = $4, blob_id = $5, created_at = $6, updated_at = $7
            "#,
            self.id,
            self.record_type,
            self.record_id,
            self.record_name,
            self.blob_id,
            self.created_at,
            self.updated_at
        )
        .execute(executor).await.map(|_|()).map_err(Into::into)
    }
}
