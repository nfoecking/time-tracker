use chrono::{DateTime, Utc};

use super::models::Tracking;

pub trait TimeRepository {
    fn init_repository(&self) -> Result<(), TimeRepositoryError>;
    fn start_tracking(&self, ts: &DateTime<Utc>) -> Result<(), TimeRepositoryError>;
    fn get_active_tracking(&self) -> Result<Option<Tracking>, TimeRepositoryError>;
    fn end_tracking(&self, id: u64, ts: &DateTime<Utc>, comment: &Option<String>, time_seconds: u64) -> Result<(), TimeRepositoryError>;
}

#[derive(Debug)]
pub enum TimeRepositoryError {
    Connection,
}
