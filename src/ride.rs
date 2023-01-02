use crate::fare_calculator_factory;
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
            let fare_calculator =
                fare_calculator_factory::FareCalculatorFactory::create(segment).unwrap();
            fare += fare_calculator.calculate(segment);
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
