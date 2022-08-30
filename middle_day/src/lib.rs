use chrono::{TimeZone, Utc,  Datelike, NaiveDate};


pub fn middle_day(year: i32) -> Option<String> {
    let days = ndays_in_year(year);
    let md = days / 2 + 1;
    if md == 366 {
        return None;
    }
    let wd = Utc.yo(year, md).to_string();
    return Some(wd);
}
fn ndays_in_year(year: i32) -> u32 {
    let d = NaiveDate::from_ymd(year + 1, 1, 1);
    d.pred().ordinal()
}