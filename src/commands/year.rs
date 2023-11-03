use chrono::{Utc, Duration};
use cli_table::{Table, print_stdout, WithTitle};

use crate::{helper::datetime, domain::{repositories::TimeRepository, models::TrackingYearAggregation}};

pub fn year_command(time_repository: Box<dyn TimeRepository>, ts: &Option<String>){
    let dt = match ts {
        Some(v) => datetime::parse_ts(v),
        None => Some(Utc::now())
    };

    if dt.is_none() {
        eprintln!("Not able to parse the given datetime. Please use a timestamp in the following format: 2023-09-08T23:06");
        return
    }

    let aggs = match time_repository.get_aggregation_for_year(&dt.unwrap()){
        Ok(aggs) => aggs,
        Err(_) => {
            eprintln!("Failed to load monthly aggregated data");
            return;
        }
    };

    let table_aggs: Vec<YearAggTableEntry> = aggs.iter().map(YearAggTableEntry::from).collect();
    let res = print_stdout(table_aggs.with_title());
    
    if let Err(_) = res {
        eprintln!("Failed to print result to stdout");
    }

    let overall_duration: i64 = aggs.iter().map(|t| {t.time_seconds as i64}).sum();

    println!("Overall duration is {}", datetime::format_duration(Duration::seconds(overall_duration)));
}


#[derive(Table)]
struct YearAggTableEntry {
    #[table(title = "month")]
    month: String,
    #[table(title = "time")]
    time: String
}

impl YearAggTableEntry {
    fn from(agg: &TrackingYearAggregation) -> Self {
        let time = datetime::format_duration(Duration::seconds(agg.time_seconds as i64));

        return Self { month: datetime::format_month(&agg.month),  time: time }
    }
}