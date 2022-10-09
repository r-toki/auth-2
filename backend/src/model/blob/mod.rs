mod repository;

use super::lib::{date_time::get_current_date_time, id::get_new_id};

use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Blob {
    pub id: String,
    pub data: Vec<u8>,
    pub name: String,
    pub content_type: String,
    pub byte_size: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Blob {
    pub fn create(encoded: String, name: String, content_type: String) -> anyhow::Result<Self> {
        let id = get_new_id();
        let now = get_current_date_time();

        let decoded = base64::decode(encoded)?;
        let byte_size: i32 = decoded.len().try_into()?;

        let blob = Blob::new(id, decoded, name, content_type, byte_size, now, now);
        Ok(blob)
    }
}
