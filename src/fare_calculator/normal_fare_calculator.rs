use super::{FareCalculator, Segment};

const NORMAL_FARE: f64 = 2.1;

pub struct NormalFareCalculator {
    fare: f64,
}

impl NormalFareCalculator {
    pub fn new() -> Self {
        Self { fare: NORMAL_FARE }
    }
}

impl FareCalculator for NormalFareCalculator {
    fn calculate(&self, segment: &Segment) -> f64 {
        segment.distance() as f64 * self.fare
    }
}
