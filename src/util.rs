use chrono::{DateTime, Utc};

pub trait ToDate {
    fn to_date_from_timestamp(&self) -> Option<DateTime<Utc>>;
}

impl ToDate for u64 {
    fn to_date_from_timestamp(&self) -> Option<DateTime<Utc>> {
        let seconds = self / 1000;
        let nanoseconds = (self % 1000) * 1_000_000;

        DateTime::from_timestamp(seconds as i64, nanoseconds as u32)
    }
}
