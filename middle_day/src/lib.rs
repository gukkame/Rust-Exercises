pub use chrono::*;
pub use chrono::{Weekday as wd, DateTime, offset::Utc};
fn main() {
    println!("{:?}", middle_day(1022).unwrap());
}

pub fn middle_day(year: i32) -> Option<wd> {
    if year % 4 == 0 {
        return None;
    } else {
        let day = Utc.yo(year, 183).weekday();
        return Some(day);
    }
}