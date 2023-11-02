use chrono::{DateTime, Utc, NaiveDateTime, TimeZone, LocalResult};

use crate::domain::repositories::TimeRepository;

pub fn start_command(time_repository: Box<dyn TimeRepository>, ts: &Option<String>){
    if tracking_is_active(&time_repository) {
        eprintln!("Please stop the active tracking before starting a new one.");
        return;
    }


    let dt = match ts {
        Some(v) => parse_ts(v),
        None => Some(Utc::now())
    };

    if dt.is_none() {
        eprintln!("Not able to parse the given datetime. Please use a timestamp in the following format: 2023-09-08T23:06");
        return
    }

    let repo_result = time_repository.start_tracking(&(dt.unwrap()));
    match repo_result {
        Ok(()) => println!("Successfully started a new time tracking"),
        Err(_) => eprintln!("Failed to store the given data in the database")
    }
}

fn tracking_is_active(time_repository: &Box<dyn TimeRepository>) -> bool {
    match time_repository.get_active_tracking() {
        Ok(Some(_)) => true,
        Err(_) => true,
        _ => false
    }
}

fn parse_ts(date_str: &str) -> Option<DateTime<Utc>>{
    let naive_dt = NaiveDateTime::parse_from_str(date_str, "%FT%R").ok()?;
    match Utc.from_local_datetime(&naive_dt) {
        LocalResult::Single(v) => Some(v),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::parse_ts;

    #[test]
    fn datetime_test() {
        let result = parse_ts("2023-01-01T07:30");
        assert!(result.is_some())
    }
}