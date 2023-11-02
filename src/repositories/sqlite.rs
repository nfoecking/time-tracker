use sqlite::{Connection, Error};

use crate::domain::repositories::{TimeRepository, TimeRepositoryError};

pub struct SqliteTimeRepository {
    connection: Connection,
}

impl SqliteTimeRepository {
    pub fn new() -> Result<SqliteTimeRepository, TimeRepositoryError> {
        let conn = Connection::open("time-tracker.db")?;

        Ok(SqliteTimeRepository { connection: conn })
    }
}

impl TimeRepository for SqliteTimeRepository {
    fn init_repository(&self) -> Result<(), TimeRepositoryError> {
        self.connection.execute(
            "create table if not exists times (
                id integer primary key,
                comment text,
                start_ts datetime not null,
                end_ts datetime,
                time_seconds integer not null
            )",
        )?;

        Ok(())
    }
}

impl From<Error> for TimeRepositoryError {
    fn from(_inner: Error) -> Self {
        TimeRepositoryError::Connection
    }
}
