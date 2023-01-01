use crate::segment::Segment;
use chrono::prelude::*;

const NORMAL_FARE: f64 = 2.1;
const OVERNIGHT_FARE: f64 = 3.9;
const SUNDAY_FARE: f64 = 2.9;
const OVERNIGHT_SUNDAY_FARE: f64 = 5.0;
const FIRST_DAY_FARE: f64 = 1.5;
const MIN_FARE: f64 = 10.0;

pub struct Ride {
    segments: Vec<Segment>,
}

impl Default for Ride {
    fn default() -> Self {
        Self::new()
    }
}

impl Ride {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, distance: u32, date: DateTime<Utc>) {
        let segment = Segment::new(distance, date);
        self.segments.push(segment)
    }

    pub fn calculate_ride(&self) -> f64 {
        let mut fare: f64 = 0.0;

        for segment in &self.segments {
            if segment.is_special_day() {
                fare += segment.distance() as f64 * FIRST_DAY_FARE;
                continue;
            }
            if segment.is_overnight() && !segment.is_sunday() {
                fare += segment.distance() as f64 * OVERNIGHT_FARE;
                continue;
            }
            if segment.is_overnight() && segment.is_sunday() {
                fare += segment.distance() as f64 * OVERNIGHT_SUNDAY_FARE;
                continue;
            }
            if !segment.is_overnight() && segment.is_sunday() {
                fare += segment.distance() as f64 * SUNDAY_FARE;
                continue;
            }

            if !segment.is_overnight() && !segment.is_sunday() {
                fare += segment.distance() as f64 * NORMAL_FARE;
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
}
