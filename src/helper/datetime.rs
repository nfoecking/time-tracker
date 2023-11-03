use chrono::{DateTime, Utc, NaiveDateTime, LocalResult, TimeZone};

pub fn parse_ts(date_str: &str) -> Option<DateTime<Utc>>{
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