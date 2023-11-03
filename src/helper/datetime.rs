use chrono::{DateTime, Utc, NaiveDateTime, LocalResult, TimeZone, Duration};

pub fn parse_ts(date_str: &str) -> Option<DateTime<Utc>>{
    let naive_dt = NaiveDateTime::parse_from_str(date_str, "%FT%R").ok()?;
    match Utc.from_local_datetime(&naive_dt) {
        LocalResult::Single(v) => Some(v),
        _ => None
    }
}

pub fn format_ts(ts: &Option<DateTime<Utc>>) -> String {
    match ts {
        None => String::from("None"),
        Some(v) => v.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

pub fn format_duration(d: Duration) -> String {
    let total_seconds = d.num_seconds();
    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let hours = (total_seconds / 60) / 60;
    format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
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