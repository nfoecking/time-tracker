use chrono::Utc;

use crate::{domain::repositories::TimeRepository, helper::datetime};

pub fn end_command(time_repository: Box<dyn TimeRepository>, ts: &Option<String>, comment: &Option<String>){
    let active_tracking = match time_repository.get_active_tracking() {
        Ok(Some(t)) => t,
        Err(_) => {
            eprintln!("An error occurred when loading the active tracking");
            return;
        },
        Ok(None) => {
            eprintln!("Please first start a tracking before ending one");
            return;
        }
    };

    let end_ts = match ts {
        Some(v) => datetime::parse_ts(v),
        None => Some(Utc::now())
    };

    if end_ts.is_none() {
        eprintln!("Not able to parse the given datetime. Please use a timestamp in the following format: 2023-09-08T23:06");
        return;
    }

    let end_ts = end_ts.unwrap();

    let time_seconds = (end_ts - active_tracking.start_ts).num_seconds();
    if time_seconds < 0 {
        eprintln!("The given end_ts needs to be larger than the start_ts");
        return;
    }
    let time_seconds = time_seconds as u64;

    let result = time_repository.end_tracking(active_tracking.id, &end_ts, comment, time_seconds);
    match result {
        Ok(_) => println!("Successfully ended the active tracking."),
        Err(_) => eprintln!("Failed to end the active tracking.")
    };
}