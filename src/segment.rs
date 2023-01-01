use chrono::prelude::*;

const SPECIAL_DAY: u32 = 1;
const OVERNIGHT_END: u32 = 6;
const OVERNIGHT_START: u32 = 22;

pub struct Segment {
    distance: u32,
    date: DateTime<Utc>,
}

impl Segment {
    pub fn new(distance: u32, date: DateTime<Utc>) -> Self {
        Self { distance, date }
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn is_special_day(&self) -> bool {
        self.date.day() == SPECIAL_DAY
    }

    pub fn is_sunday(&self) -> bool {
        self.date.weekday() == Weekday::Sun
    }

    pub fn is_overnight(&self) -> bool {
        let hour = self.date.hour();
        hour >= OVERNIGHT_START || hour <= OVERNIGHT_END
    }
}
