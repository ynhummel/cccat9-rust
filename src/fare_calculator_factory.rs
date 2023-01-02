use crate::fare_calculator::{FareCalculator, *};
use crate::segment::Segment;

pub struct FareCalculatorFactory {}

impl FareCalculatorFactory {
    pub fn create(segment: &Segment) -> Result<Box<dyn FareCalculator>, CalculatorNotFound> {
        if segment.is_special_day() {
            return Ok(Box::new(
                special_day_fare_calculator::SpecialDayFareCalculator::new(),
            ));
        }
        if segment.is_overnight() && !segment.is_sunday() {
            return Ok(Box::new(
                overnight_fare_calculator::OvernightFareCalculator::new(),
            ));
        }
        if segment.is_overnight() && segment.is_sunday() {
            return Ok(Box::new(
                overnight_sunday_fare_calculator::OvernightSundayFareCalculator::new(),
            ));
        }
        if !segment.is_overnight() && segment.is_sunday() {
            return Ok(Box::new(sunday_fare_calculator::SundayFareCalculator::new()));
        }
        if !segment.is_overnight() && !segment.is_sunday() {
            return Ok(Box::new(normal_fare_calculator::NormalFareCalculator::new()));
        }
        Err(CalculatorNotFound)
    }
}
