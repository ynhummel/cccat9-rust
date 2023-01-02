use super::{FareCalculator, Segment};

const OVERNIGHT_SUNDAY_FARE: f64 = 5.0;

pub struct OvernightSundayFareCalculator {
    fare: f64,
}

impl OvernightSundayFareCalculator {
    pub fn new() -> Self {
        Self {
            fare: OVERNIGHT_SUNDAY_FARE,
        }
    }
}

impl FareCalculator for OvernightSundayFareCalculator {
    fn calculate(&self, segment: &Segment) -> f64 {
        segment.distance() as f64 * self.fare
    }
}
