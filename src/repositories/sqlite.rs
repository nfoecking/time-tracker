use chrono::{DateTime, Utc};
use rusqlite::{params, types::Null, Connection, Error};

use crate::domain::{
    models::Tracking,
    repositories::{TimeRepository, TimeRepositoryError},
};

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
            (),
        )?;

        Ok(())
    }

    fn start_tracking(&self, ts: &DateTime<Utc>) -> Result<(), TimeRepositoryError> {
        self.connection.execute(
            "insert into times (comment, start_ts, end_ts, time_seconds) values (?1, ?2, ?3, ?4)",
            params![Null, ts, Null, 0],
        )?;

        Ok(())
    }

    fn get_active_tracking(&self) -> Result<Option<Tracking>, TimeRepositoryError> {
        let mut stmt = self.connection.prepare("SELECT id, comment, start_ts, end_ts, time_seconds FROM times WHERE end_ts IS NULL")?;
        let tracking_iter = stmt.query_map([], |row| {
            Ok(Tracking {
                id: row.get(0)?,
                comment: row.get(1)?,
                start_ts: row.get(2)?,
                end_ts: row.get(3)?,
                time_seconds: row.get(4)?,
            })
        })?;


        for tracking in tracking_iter {
            match tracking {
                Ok(t) => return Ok(Some(t)),
                Err(e) => return Err(e.into())
            };
        }

        Ok(None)
    }
}

impl From<Error> for TimeRepositoryError {
    fn from(_inner: Error) -> Self {
        TimeRepositoryError::Connection
    }
}
