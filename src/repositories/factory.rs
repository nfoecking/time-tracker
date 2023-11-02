use crate::domain::repositories::{TimeRepository, TimeRepositoryError};

use super::sqlite::SqliteTimeRepository;

pub fn get_time_repository() -> Result<Box<dyn TimeRepository>, TimeRepositoryError> {
    // here could be other repository implementations in the future
    let sqlite_repo = SqliteTimeRepository::new()?;

    Ok(Box::new(sqlite_repo))
}