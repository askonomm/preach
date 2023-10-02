use chrono::{DateTime, Utc};
use std::time::SystemTime;
use time::{format_description::well_known::Iso8601, PrimitiveDateTime};

pub fn date_str_to_system_time(date_str: &str) -> SystemTime {
    let date_str = format!("{}T00:00:00", date_str);
    let primitive_datetime = PrimitiveDateTime::parse(&date_str, &Iso8601::DEFAULT).unwrap();
    let unix_timestamp = primitive_datetime.assume_utc().unix_timestamp();

    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(unix_timestamp as u64)
}

pub fn system_time_to_date_str(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = system_time.into();

    datetime.to_rfc3339()
}
