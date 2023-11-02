use chrono::{DateTime, Utc};

use super::models::Tracking;

pub trait TimeRepository {
    fn init_repository(&self) -> Result<(), TimeRepositoryError>;
    fn start_tracking(&self, ts: &DateTime<Utc>) -> Result<(), TimeRepositoryError>;
    fn get_active_tracking(&self) -> Result<Option<Tracking>, TimeRepositoryError>;
}

#[derive(Debug)]
pub enum TimeRepositoryError {
    Connection,
}
