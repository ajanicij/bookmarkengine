use chrono::{DateTime, Utc};
use std::error::Error;

pub fn date_time_from_str(ts_str: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let ts: i64 = ts_str.parse::<i64>()?;
    let dt = DateTime::from_timestamp(ts, 0).ok_or(format!("bad timestamp: {}", ts_str))?;
    Ok(dt)
}

pub fn days_from(dt: DateTime<Utc>) -> i64 {
    let now = Utc::now();
    let duration = now - dt;
    let days = duration.num_days();
    days
}
