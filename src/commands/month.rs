use chrono::{Utc, Duration};
use cli_table::{Table, print_stdout, WithTitle};

use crate::{helper::datetime, domain::{repositories::TimeRepository, models::TrackingMonthAggregation}};

pub fn month_command(time_repository: Box<dyn TimeRepository>, ts: &Option<String>){
    let dt = match ts {
        Some(v) => datetime::parse_ts(v),
        None => Some(Utc::now())
    };

    if dt.is_none() {
        eprintln!("Not able to parse the given datetime. Please use a timestamp in the following format: 2023-09-08T23:06");
        return
    }

    let aggs = match time_repository.get_aggregation_for_month(&dt.unwrap()){
        Ok(aggs) => aggs,
        Err(_) => {
            eprintln!("Failed to load daily aggregated data");
            return;
        }
    };

    let table_aggs: Vec<MonthAggTableEntry> = aggs.iter().map(MonthAggTableEntry::from).collect();
    let res = print_stdout(table_aggs.with_title());
    
    if let Err(_) = res {
        eprintln!("Failed to print result to stdout");
    }

    let overall_duration: i64 = aggs.iter().map(|t| {t.time_seconds as i64}).sum();

    println!("Overall duration is {}", datetime::format_duration(Duration::seconds(overall_duration)));
}


#[derive(Table)]
struct MonthAggTableEntry {
    #[table(title = "date")]
    date: String,
    #[table(title = "time")]
    time: String,
    #[table(title = "time (hours)")]
    time_decimal: String
}

impl MonthAggTableEntry {
    fn from(agg: &TrackingMonthAggregation) -> Self {
        let time = datetime::format_duration(Duration::seconds(agg.time_seconds as i64));
        let time_decimal = format!("{:.2}", (agg.time_seconds as f64 / 3600.0));

        return Self { date: datetime::format_date(&agg.date),  time: time, time_decimal: time_decimal }
    }
}