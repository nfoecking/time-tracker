use chrono::{Utc, Duration};
use cli_table::{print_stdout, Table, WithTitle};
use crate::{domain::{repositories::TimeRepository, models::Tracking}, helper::datetime};

pub fn day_command(time_repository: Box<dyn TimeRepository>, ts: &Option<String>){
    let dt = match ts {
        Some(v) => datetime::parse_ts(v),
        None => Some(Utc::now())
    };

    if dt.is_none() {
        eprintln!("Not able to parse the given datetime. Please use a timestamp in the following format: 2023-09-08T23:06");
        return
    }

    let trackings = match time_repository.get_trackings_of_date(dt.unwrap().date_naive()){
        Err(_) => {
            eprintln!("Failed to load daily trackings");
            return;
        },
        Ok(t) => t
    };

    let table_trackings: Vec<TrackingTableEntry> = trackings.iter().map(TrackingTableEntry::from).collect();
    let res = print_stdout(table_trackings.with_title());
    
    if let Err(_) = res {
        eprintln!("Failed to print result to stdout");
    }

    let overall_duration: i64 = trackings.iter().map(|t| {
        match t.end_ts {
            Some(_) => t.time_seconds as i64,
            None => (Utc::now() - t.start_ts).num_seconds()
        }
    }).sum();

    println!("Overall duration is {}", datetime::format_duration(Duration::seconds(overall_duration)));
}

#[derive(Table)]
struct TrackingTableEntry {
    #[table(title = "start ts")]
    start_ts: String,
    #[table(title = "end ts")]
    end_ts: String,
    #[table(title = "comment")]
    comment: String,
    #[table(title = "time")]
    time: String
}

impl TrackingTableEntry {
    fn from(tracking: &Tracking) -> Self {
        let comment = match &tracking.comment {
            Some(c) => c,
            None => ""
        };

        let time = match tracking.end_ts {
            Some(_) => {
                let duration = Duration::seconds(tracking.time_seconds as i64);
                datetime::format_duration(duration)
            },
            None => {
                let mut duration_str = datetime::format_duration(Utc::now() - tracking.start_ts);
                duration_str.push_str(" (NOT FINISHED)");
                duration_str
            }
        };

        return Self { start_ts: datetime::format_ts(&Some(tracking.start_ts)), end_ts: datetime::format_ts(&tracking.end_ts), comment: comment.to_owned(), time: time }
    }
}