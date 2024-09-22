use chrono::NaiveDate;

pub trait ToDate {
    fn to_date(&self) -> Option<NaiveDate>;
}

impl ToDate for u64 {
    fn to_date(&self) -> Option<NaiveDate> {
        let year = (self / 10000) as i32;
        let month = ((self / 100) % 100) as u32;
        let day = (self % 100) as u32;

        NaiveDate::from_ymd_opt(year, month, day)
    }
}
