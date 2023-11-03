use chrono::{DateTime, Utc, NaiveDate, Days};
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
        let mut stmt = self.connection.prepare(
            "SELECT id, comment, start_ts, end_ts, time_seconds FROM times WHERE end_ts IS NULL",
        )?;
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
                Err(e) => return Err(e.into()),
            };
        }

        Ok(None)
    }

    fn end_tracking(
        &self,
        id: u64,
        ts: &DateTime<Utc>,
        comment: &Option<String>,
        time_seconds: u64
    ) -> Result<(), TimeRepositoryError> {
        let comment_set = match comment {
            Some(_) => " comment = ?1, ",
            None => " "
        };
        self.connection.execute(
            &format!("UPDATE times SET{}end_ts=?2, time_seconds=?4 WHERE id=?3", comment_set),
            params![comment.as_deref().unwrap_or(""), ts, id, time_seconds],
        )?;

        Ok(())
    }

    fn get_trackings_of_date(&self, date: NaiveDate)-> Result<Vec<Tracking>, TimeRepositoryError>{
        let next_date = date.checked_add_days(Days::new(1)).unwrap_or(date);
        let mut stmt = self.connection.prepare(
            "SELECT id, comment, start_ts, end_ts, time_seconds FROM times WHERE start_ts >= ?1 and start_ts < ?2",
        )?;
        let tracking_iter = stmt.query_map([date, next_date], |row| {
            Ok(Tracking {
                id: row.get(0)?,
                comment: row.get(1)?,
                start_ts: row.get(2)?,
                end_ts: row.get(3)?,
                time_seconds: row.get(4)?,
            })
        })?;

        let mut trackings = Vec::new();

        for tracking in tracking_iter {
            match tracking {
                Ok(t) => trackings.push(t),
                Err(e) => return Err(e.into()),
            };
        }

        Ok(trackings)

    }
}

impl From<Error> for TimeRepositoryError {
    fn from(_inner: Error) -> Self {
        TimeRepositoryError::Connection
    }
}
