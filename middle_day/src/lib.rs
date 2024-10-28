use chrono::{NaiveDate, Datelike, Weekday};


#[derive(Debug, PartialEq, Eq)]
pub enum wd{
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}
pub fn middle_day(year: i32) -> Option<wd> {
   let number_of_day_in_year = if  NaiveDate::from_ymd_opt(year, 2,29).is_some() {366}else{365};
   if number_of_day_in_year % 2 == 0 {
       None
   }else{
    let middle_day: u32 = number_of_day_in_year / 2 + 1 ;
    let middle_date = NaiveDate::from_yo(year, middle_day);
    let day = middle_date.weekday();
    match day {
       Weekday::Mon => Some(wd::Mon),
       Weekday::Wed => Some(wd::Wed),
       Weekday::Thu => Some(wd::Thu),
       Weekday::Tue => Some(wd::Tue),
       Weekday::Fri => Some(wd::Fri),
       Weekday::Sat => Some(wd::Sat),
       Weekday::Sun => Some(wd::Sun),
    }
   }
}
