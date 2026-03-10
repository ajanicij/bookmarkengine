use chrono::{DateTime, Utc, TimeZone};

/*
fn main() {
    println!("Hello, world!");
    let add_date = "1741583315";
    let epoch_time: u32 = add_date.parse::<u32>().expect("Bad epoch time!");
    println!("epoch time: {}", epoch_time);

    let date_time = chrono::DateTime::from_timestamp(epoch_time as i64, 0).expect("Bad timestamp");
    let formatted = format!("{}", date_time.format("%d/%m/%Y %H:%M"));
    println!("formatted GMT: {}", formatted);

    let now = chrono::Local::now();
    let tz_offset = now.offset();
    println!("Timezone offset: {}", tz_offset);
    let localized_date_time = date_time.with_timezone(tz_offset);
    let local_formatted = format!("{}", localized_date_time.format("%d/%m/%Y %H:%M"));
    println!("formatted local time: {}", local_formatted);

    let duration = now.with_timezone(tz_offset) - localized_date_time;
    let days = duration.num_days();
    println!("number of days: {}", days);
}
*/

fn main() {
    let epoch: i64 = 1741583315;

    let days = days_from_epoch(epoch);

    println!("Days difference: {}", days);
}

pub fn days_from_epoch(epoch_seconds: i64) -> i64 {
    let then: DateTime<Utc> = Utc.timestamp_opt(epoch_seconds, 0)
        .single()
        .expect("Invalid epoch timestamp");

    let now: DateTime<Utc> = Utc::now();

    now.signed_duration_since(then).num_days()
}