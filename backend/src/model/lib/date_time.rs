use chrono::{DateTime, Utc};

pub fn get_current_date_time() -> DateTime<Utc> {
    Utc::now()
}
