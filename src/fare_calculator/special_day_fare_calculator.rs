use super::{FareCalculator, Segment};

const SPECIAL_DAY_FARE: f64 = 1.5;

pub struct SpecialDayFareCalculator {
    fare: f64,
}

impl SpecialDayFareCalculator {
    pub fn new() -> Self {
        Self {
            fare: SPECIAL_DAY_FARE,
        }
    }
}

impl FareCalculator for SpecialDayFareCalculator {
    fn calculate(&self, segment: &Segment) -> f64 {
        segment.distance() as f64 * self.fare
    }
}
