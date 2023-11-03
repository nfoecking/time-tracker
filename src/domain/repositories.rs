use chrono::{DateTime, Utc, NaiveDate};

use super::models::{Tracking, TrackingMonthAggregation};

pub trait TimeRepository {
    fn init_repository(&self) -> Result<(), TimeRepositoryError>;
    fn start_tracking(&self, ts: &DateTime<Utc>) -> Result<(), TimeRepositoryError>;
    fn get_active_tracking(&self) -> Result<Option<Tracking>, TimeRepositoryError>;
    fn end_tracking(&self, id: u64, ts: &DateTime<Utc>, comment: &Option<String>, time_seconds: u64) -> Result<(), TimeRepositoryError>;
    fn get_trackings_of_date(&self, date: NaiveDate) -> Result<Vec<Tracking>, TimeRepositoryError>;
    fn get_aggregation_for_month(&self, ts: &DateTime<Utc>) -> Result<Vec<TrackingMonthAggregation>, TimeRepositoryError>;
}

#[derive(Debug)]
pub enum TimeRepositoryError {
    Connection,
}
