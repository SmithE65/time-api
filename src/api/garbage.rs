use actix_web::{get, HttpResponse, Responder};
use chrono::{Datelike, Local};

#[get("/garbage")]
async fn responder() -> impl Responder {
    let local = Local::now().timestamp_millis();
    let days_until_garbage = get_days_until_garbage(local);
    let days_until_recycling = get_days_until_recycling(local);
    let week_number = Local::now().iso_week().week();
    let stable_week = get_weeks_since_first_sunday(local);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!(
            "{{\"t\":{},\"g\":{},\"r\":{},\"w\":{},\"s\":{}}}",
            local, days_until_garbage, days_until_recycling, week_number, stable_week
        ))
}

const MS_PER_HOUR: i64 = 60 * 60 * 1000;
const MS_PER_DAY: i64 = MS_PER_HOUR * 24;
const RECYCLING_PERIOD: i64 = 14;
const GARBAGE_PERIOD: i64 = 7;
const WEEK: i64 = 7;
const THURSDAY: i64 = 4;
const TUESDAY: i64 = 2;
const EPOCH_OFFSET: i64 = 4;
const RECYCLING_OFFSET: i64 = EPOCH_OFFSET + WEEK;
const OFFSET_TO_SUNDAY: i64 = 3;

fn get_days_until_garbage(ms_since_epoch: i64) -> i64 {
    let day_of_week = ((ms_since_epoch / MS_PER_DAY) + EPOCH_OFFSET) % GARBAGE_PERIOD;
    (GARBAGE_PERIOD + THURSDAY - day_of_week) % GARBAGE_PERIOD
}

fn get_days_until_recycling(ms_since_epoch: i64) -> i64 {
    let day_of_biweek = ((ms_since_epoch / MS_PER_DAY) + RECYCLING_OFFSET) % RECYCLING_PERIOD;
    (RECYCLING_PERIOD + TUESDAY - day_of_biweek) % RECYCLING_PERIOD
}

fn get_weeks_since_first_sunday(ms_since_epoch: i64) -> i64 {
    ((ms_since_epoch / MS_PER_DAY) + OFFSET_TO_SUNDAY) / WEEK
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Local};

    // Helper to generate a timestamp for 6am local time (to avoid midnight weirdness)
    fn ts(y: i32, m: u32, d: u32) -> i64 {
        Local.with_ymd_and_hms(y, m, d, 6, 0, 0).unwrap().timestamp_millis()
    }

    #[test]
    fn test_garbage_schedule_thursday() {
        // Jan 4, 2024 is a Thursday
        let t = ts(2024, 1, 4);
        assert_eq!(get_days_until_garbage(t), 0);
    }

    #[test]
    fn test_garbage_schedule_before_thursday() {
        let t = ts(2024, 1, 3); // Wednesday
        assert_eq!(get_days_until_garbage(t), 1);
    }

    #[test]
    fn test_garbage_schedule_after_thursday() {
        let t = ts(2024, 1, 5); // Friday
        assert_eq!(get_days_until_garbage(t), 6);
    }

    #[test]
    fn test_recycling_schedule_tuesday() {
        let t = ts(2024, 1, 9); // Known Tuesday
        assert_eq!(get_days_until_recycling(t), 0);
    }

    #[test]
    fn test_week1_started_jan4() {
        let start = ts(1970, 1, 4); // First Sunday
        let result = get_weeks_since_first_sunday(start);
        assert_eq!(result, 0);
    }
    
    #[test]
    fn test_week1_started_jan11() {
        let start = ts(1970, 1, 11); // First Sunday
        let result = get_weeks_since_first_sunday(start);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_week_stability_even_vs_odd() {
        let start = ts(2024, 1, 7); // Sunday
        let one_week = MS_PER_DAY * 7;

        let week_0 = get_weeks_since_first_sunday(start);
        let week_1 = get_weeks_since_first_sunday(start + one_week);
        let week_2 = get_weeks_since_first_sunday(start + 2 * one_week);

        assert_eq!(week_1, week_0 + 1);
        assert_eq!(week_2, week_0 + 2);
        assert_eq!(week_0 % 2, 0); // If Jan 7 is week 0
    }
}
