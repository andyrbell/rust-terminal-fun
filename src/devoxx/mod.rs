pub mod model;
use std::fs;
use crate::devoxx::model::{ScheduleItem};
use chrono::{Weekday, DateTime, Utc};
use std::str::FromStr;

pub fn get_talks_by_weekday(day: &Weekday) -> Result<Vec<ScheduleItem>, failure::Error> {
    let day = match day {
        Weekday::Mon => "monday",
        Weekday::Tue => "tuesday",
        Weekday::Wed => "wednesday",
        Weekday::Thu => "thursday",
        Weekday::Fri => "friday",
        _ => "monday"
    };

    get_talks_by_day(day)
}

pub fn get_talks_by_day(day: &str) -> Result<Vec<ScheduleItem>, failure::Error> {
    let contents = fs::read_to_string(format!("devoxx-data/{}.json", day))?;
    serde_json::from_str(&contents)
        .map_err(failure::Error::from)
}

#[derive(Debug)]
struct ScheduleItem2 {
    talk_title: String,
    from_date: DateTime<Utc>
}

pub fn foo() {
    let content  = fs::read_to_string("./devoxx-data/talks.txt").unwrap();
    let lines = content.lines().map(to_schedule).collect::<Vec<ScheduleItem2>>();
    eprintln!("lines = {:#?}", lines);
    for line in lines {

    }

}

fn to_schedule(line :&str) -> ScheduleItem2 {
    let foo = line.split(",").collect::<Vec<&str>>();
    let talk_title = foo.get(0).unwrap().to_string();
    let from_date = foo.get(1).unwrap();
//    eprintln!("title = {}", talk_title);
//    eprintln!("date = {}", from_date);
    let from_date = DateTime::from_str(from_date).expect("should be a date");
    ScheduleItem2 { talk_title, from_date }

//    eprintln!("scedule = {:#?}", schedule);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let talks = get_talks_by_day(&"monday".to_string());
        assert_eq!(true, talks.is_ok());
        if let Ok(talks) = talks {
            assert_eq!(talks.len(), 38);
        }
    }

    #[test]
    fn test2() {
        foo();
    }
}
