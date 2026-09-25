use chrono::{Datelike, TimeZone};

use crate::core::time::TDuration;

#[must_use]
pub fn time_in_day_left() -> TDuration {
    let now = chrono::Local::now();
    let end_of_day = chrono::Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 23, 59, 59)
        .unwrap();
    let time_passed = end_of_day.signed_duration_since(now).num_seconds() as f64;

    time_passed.into()
}
