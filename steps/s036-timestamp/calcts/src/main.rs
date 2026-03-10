use chrono::{DateTime, Utc};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let ts_str = "1741583963";

    let dt = date_time_from_str(ts_str)?;
    println!("time of the specified epoch time: {}", dt.format("%Y-%m-%d %H:%M"));

    let days = days_from(dt);
    println!("days from specified epoch time: {}", days);

    let days = days_from_str(ts_str)?;
    println!("days from specified epoch time (as days_from_str returns it): {}", days);

    Ok(())
}

fn date_time_from_str(ts_str: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let ts: i64 = ts_str.parse::<i64>()?;
    let dt = DateTime::from_timestamp(ts, 0).ok_or(format!("bad timestamp: {}", ts_str))?;
    Ok(dt)
}

fn days_from(dt: DateTime<Utc>) -> i64 {
    let now = Utc::now();
    let duration = now - dt;
    let days = duration.num_days();
    days
}

fn days_from_str(ts_str: &str) -> Result<i64, Box<dyn Error>> {
    let dt = date_time_from_str(ts_str)?;
    Ok(days_from(dt))
}