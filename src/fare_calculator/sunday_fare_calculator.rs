use super::{FareCalculator, Segment};

const SUNDAY_FARE: f64 = 2.9;

pub struct SundayFareCalculator {
    fare: f64,
}

impl SundayFareCalculator {
    pub fn new() -> Self {
        Self { fare: SUNDAY_FARE }
    }
}

impl FareCalculator for SundayFareCalculator {
    fn calculate(&self, segment: &Segment) -> f64 {
        segment.distance() as f64 * self.fare
    }
}
