use chrono::{DateTime, Utc};

pub struct Tracking {
    pub start_ts: DateTime<Utc>,
    pub end_ts: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub time_seconds: u64,
    pub id: u64
}