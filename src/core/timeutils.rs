use chrono::{Datelike, TimeZone};

use crate::core::{context::Ctx, time::TDuration};

#[must_use]
pub fn time_in_day_left(ctx: &mut Ctx) -> TDuration {
    let now = chrono::Local::now();

    let eod: (u32, u32, u32) = if let Ok(cfg) = ctx.config()
        && let Some(opts) = cfg.options()
        && let Some(eod) = opts.day_end()
    {
        (
            eod.hours() as u32,
            eod.minutes() as u32,
            eod.seconds() as u32,
        )
    } else {
        (23, 59, 59)
    };

    let end_of_day = chrono::Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), eod.0, eod.1, eod.2)
        .unwrap();

    let time_passed = end_of_day.signed_duration_since(now).num_seconds() as f64;

    time_passed.into()
}
