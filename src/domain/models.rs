use chrono::{DateTime, Utc, NaiveDate};

pub struct Tracking {
    pub start_ts: DateTime<Utc>,
    pub end_ts: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub time_seconds: u64,
    pub id: u64
}

pub struct TrackingMonthAggregation {
    pub date: NaiveDate,
    pub time_seconds: u64
}

pub struct TrackingYearAggregation {
    pub month: NaiveDate,
    pub time_seconds: u64
}