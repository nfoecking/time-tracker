use chrono::Utc;

use crate::{domain::repositories::TimeRepository, helper::datetime};

pub fn start_command(time_repository: Box<dyn TimeRepository>, ts: &Option<String>){
    if tracking_is_active(&time_repository) {
        eprintln!("Please stop the active tracking before starting a new one.");
        return;
    }


    let dt = match ts {
        Some(v) => datetime::parse_ts(v),
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