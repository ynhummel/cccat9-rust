use core::fmt;

pub use crate::segment::Segment;

pub mod normal_fare_calculator;
pub mod overnight_fare_calculator;
pub mod overnight_sunday_fare_calculator;
pub mod special_day_fare_calculator;
pub mod sunday_fare_calculator;

pub trait FareCalculator {
    fn calculate(&self, segment: &Segment) -> f64;
}

#[derive(Debug, Clone)]
pub struct CalculatorNotFound;

impl fmt::Display for CalculatorNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fare Calculator not found!")
    }
}

impl std::error::Error for CalculatorNotFound {}
