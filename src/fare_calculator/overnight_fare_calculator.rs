use super::{FareCalculator, Segment};

const OVERNIGHT_FARE: f64 = 3.9;

pub struct OvernightFareCalculator {
    fare: f64,
}

impl OvernightFareCalculator {
    pub fn new() -> Self {
        Self {
            fare: OVERNIGHT_FARE,
        }
    }
}

impl FareCalculator for OvernightFareCalculator {
    fn calculate(&self, segment: &Segment) -> f64 {
        segment.distance() as f64 * self.fare
    }
}
