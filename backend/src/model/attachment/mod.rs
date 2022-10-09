mod repository;

use super::lib::{date_time::get_current_date_time, id::get_new_id};

use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Debug, Serialize, Deserialize)]
struct Attachment {
    pub id: String,
    pub record_type: Option<String>,
    pub record_id: Option<String>,
    pub record_name: Option<String>,
    pub blob_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Attachment {
    pub fn create(
        record_type: String,
        record_id: String,
        record_name: String,
        blob_id: String,
    ) -> Self {
        let id = get_new_id();
        let now = get_current_date_time();

        Attachment::new(
            id,
            Some(record_type),
            Some(record_id),
            Some(record_name),
            Some(blob_id),
            now,
            now,
        )
    }
}
