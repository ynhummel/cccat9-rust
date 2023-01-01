mod validate_cpf;

use chrono::prelude::*;

pub struct Segment {
    distance: u32,
    date: DateTime<Utc>,
}

impl Segment {
    pub fn new(distance: u32, y: i32, m: u32, d: u32, hour: u32, min: u32, sec: u32) -> Self {
        Self {
            distance,
            date: Utc.with_ymd_and_hms(y, m, d, hour, min, sec).unwrap(),
        }
    }
}

const NORMAL_FARE: f64 = 2.1;
const OVERNIGHT_FARE: f64 = 3.9;
const SUNDAY_FARE: f64 = 2.9;
const OVERNIGHT_SUNDAY_FARE: f64 = 5.0;
const FIRST_DAY_FARE: f64 = 1.5;
const MIN_FARE: f64 = 10.0;
const SPECIAL_DAY_FARE: f64 = 1.0;

const SPECIAL_DAY: u32 = 1;
const OVERNIGHT_END: u32 = 6;
const OVERNIGHT_START: u32 = 22;

pub fn calculate_ride(segments: Vec<Segment>) -> f64 {
    let mut fare: f64 = 0.0;
    for segment in segments {
        if is_special_day(segment.date) {
            fare += segment.distance as f64 * FIRST_DAY_FARE;
            continue;
        }
        if is_overnight(segment.date) && !is_sunday(segment.date) {
            fare += segment.distance as f64 * OVERNIGHT_FARE;
            continue;
        }
        if is_overnight(segment.date) && is_sunday(segment.date) {
            fare += segment.distance as f64 * OVERNIGHT_SUNDAY_FARE;
            continue;
        }
        if !is_overnight(segment.date) && is_sunday(segment.date) {
            fare += segment.distance as f64 * SUNDAY_FARE;
            continue;
        }

        if !is_overnight(segment.date) && !is_sunday(segment.date) {
            fare += segment.distance as f64 * NORMAL_FARE;
            continue;
        }
    }
    let fare_comp = (fare * 10.0).trunc() as u32;
    let min_comp = (MIN_FARE * 10.0).trunc() as u32;

    if fare_comp > min_comp {
        fare
    } else {
        MIN_FARE
    }
}

fn is_special_day(date: DateTime<Utc>) -> bool {
    date.day() == SPECIAL_DAY
}

fn is_sunday(date: DateTime<Utc>) -> bool {
    date.weekday() == Weekday::Sun
}

fn is_overnight(date: DateTime<Utc>) -> bool {
    let hour = date.hour();
    hour >= OVERNIGHT_START || hour <= OVERNIGHT_END
}
