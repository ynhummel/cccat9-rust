use crate::fare_calculator::*;
use crate::segment::Segment;
use chrono::prelude::*;

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
                let fare_calculator = special_day_fare_calculator::SpecialDayFareCalculator::new();
                fare += fare_calculator.calculate(segment);
                continue;
            }
            if segment.is_overnight() && !segment.is_sunday() {
                let fare_calculator = overnight_fare_calculator::OvernightFareCalculator::new();
                fare += fare_calculator.calculate(segment);
                continue;
            }
            if segment.is_overnight() && segment.is_sunday() {
                let fare_calculator =
                    overnight_sunday_fare_calculator::OvernightSundayFareCalculator::new();
                fare += fare_calculator.calculate(segment);
                continue;
            }
            if !segment.is_overnight() && segment.is_sunday() {
                let fare_calculator = sunday_fare_calculator::SundayFareCalculator::new();
                fare += fare_calculator.calculate(segment);
                continue;
            }

            if !segment.is_overnight() && !segment.is_sunday() {
                let fare_calculator = normal_fare_calculator::NormalFareCalculator::new();
                fare += fare_calculator.calculate(segment);
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
