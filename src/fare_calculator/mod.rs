pub use crate::segment::Segment;

pub mod normal_fare_calculator;
pub mod overnight_fare_calculator;
pub mod overnight_sunday_fare_calculator;
pub mod special_day_fare_calculator;
pub mod sunday_fare_calculator;

pub trait FareCalculator {
    fn calculate(&self, segment: &Segment) -> f64;
}
