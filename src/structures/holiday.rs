use serde::{Serialize, Deserialize};
use chrono::{Datelike, NaiveDate, Weekday};

use crate::structures::event::Event;

use super::day::Day;


#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Holiday {
    Birthday,
    PublicHoliday,
    Vacation,
    None
}


// #####################
//       FUNCTIONS
// #####################

pub struct EventDate {
    pub event: Event,
    pub day: u8,
    pub month: u8
}
impl EventDate {
    pub fn new(e: Event, d: u8, m:u8) -> EventDate {
        EventDate {
            event: e,
            day: d,
            month: m
        }
    }
}


pub fn all_holidays() -> Vec<EventDate> {
    let mut holidays = birthdays();
    let pub_holidays = ontario_public_holidays();
    holidays.extend( pub_holidays );
    holidays
}

pub fn birthdays() -> Vec<EventDate> {
    let year = chrono::Utc::now().year() as u16;
    vec![
        EventDate::new( Event::holiday(String::from("My Birthday"), Holiday::Birthday, String::new()), 9, 9 ),
        EventDate::new( Event::holiday(String::from("Tyler Heinz's Birthday"), Holiday::Birthday, String::new()), 13, 11),
        EventDate::new( Event::holiday(String::from("Lyauren Wheatley's Birthda"), Holiday::Birthday, String::new()), 24, 3),
        EventDate::new( Event::holiday(String::from("Kaydence White's Birthday"), Holiday::Birthday, String::new()), 4, 10),
    ]
}


pub fn ontario_public_holidays() -> Vec<EventDate> {
    let year = chrono::Utc::now().year() as u16; 
    let y = year as i32;
    vec![
        EventDate::new(Event::holiday(String::from("New Years Day"), Holiday::PublicHoliday, String::from("🎉")), 1, 1),
        EventDate::new(Event::holiday(String::from("Family Day"), Holiday::PublicHoliday, String::new()), family_day(y), 2),
        EventDate::new(Event::holiday(String::from("Valentine's Day"), Holiday::PublicHoliday,String::from("❤️") ), 14, 2),
        good_friday(year),
        easter_monday(year),
        EventDate::new(Event::holiday(String::from("April Fool's Day"), Holiday::PublicHoliday, String::from("") ), 1, 4 ),
        EventDate::new(Event::holiday(String::from("Victoria Day"), Holiday::PublicHoliday, String::from("") ), victoria_day(y), 5 ),
        EventDate::new(Event::holiday(String::from("Canada Day"), Holiday::PublicHoliday, String::from("🍁") ), 1, 7),
        EventDate::new(Event::holiday(String::from("Civic Holiday"), Holiday::PublicHoliday, String::from("") ), civic_holiday(y) ,8 ),
        EventDate::new(Event::holiday(String::from("Labour Day"), Holiday::PublicHoliday, String::from("⚒️") ), labour_day(y),9 ),        
        EventDate::new(Event::holiday(String::from("Truth and Reconciliation Day"), Holiday::PublicHoliday, String::from("") ), 30,9), 
        EventDate::new(Event::holiday(String::from("Thanksgiving"), Holiday::PublicHoliday, String::from("🦃") ), thanksgiving(y),10 ),        
        EventDate::new(Event::holiday(String::from("Halloween"), Holiday::PublicHoliday, String::from("🎃") ), 31, 10),
        EventDate::new(Event::holiday(String::from("Rememberance Day"), Holiday::PublicHoliday, String::from("") ), 11, 11),        
        EventDate::new(Event::holiday(String::from("Christmas"), Holiday::PublicHoliday, String::from("🎄") ), 25, 12 ),
        EventDate::new(Event::holiday(String::from("New Years Eve"), Holiday::PublicHoliday, String::from("🎉") ), 31, 12),        
    ]
}


fn family_day(year: i32) -> u8 {
    let february = NaiveDate::from_ymd_opt(year, 2, 1).unwrap();
    let mut third_monday = february.with_month(2).unwrap().with_day(15).unwrap();
    while third_monday.weekday() != Weekday::Mon {
        third_monday = third_monday.succ_opt().unwrap();
    }
    (third_monday.day0() + 1) as u8
}

fn victoria_day(year: i32) -> u8 {
    let mut may_25 = NaiveDate::from_ymd_opt(year, 5, 25).unwrap();
    while may_25.weekday() != Weekday::Mon {
        may_25 = may_25.pred_opt().unwrap();
    }
    (may_25.day0() + 1) as u8
}

fn civic_holiday(year: i32) -> u8 {
    let august = NaiveDate::from_ymd_opt(year, 8, 1).unwrap();
    let mut first_monday = august.with_month(8).unwrap().with_day(1).unwrap();
    while first_monday.weekday() != Weekday::Mon {
        first_monday = first_monday.succ_opt().unwrap();
    }
    (first_monday.day0() + 1) as u8
}

fn labour_day(year: i32) -> u8 {
    let september = NaiveDate::from_ymd_opt(year, 9, 1).unwrap();
    let mut first_monday = september.with_month(9).unwrap().with_day(1).unwrap();
    while first_monday.weekday() != Weekday::Mon {
        first_monday = first_monday.succ_opt().unwrap();
    }
    (first_monday.day0() + 1) as u8
}

fn thanksgiving(year: i32) -> u8 {
    let mut october_14 = NaiveDate::from_ymd_opt(year, 10, 14).unwrap();
    while october_14.weekday() != Weekday::Mon {
        october_14 = october_14.pred_opt().unwrap();
    }
    (october_14.day0() + 1) as u8
}

// https://www.geeksforgeeks.org/how-to-calculate-the-easter-date-for-a-given-year-using-gauss-algorithm/
pub fn easter_monday(y: u16) -> EventDate {
    let a = y % 19;
    let b = y % 4;
    let c = y % 7;

    let p = y / 100;
    let q = (13 + 8 * p) / 25;
    let m = (15 - q + p - p / 4) % 30;
    let n = (4 + p - p / 4) % 7;
    let d = (19 * a + m) % 30;
    let e = (2 * b + 4 * c + 6 * d + n) % 7;
    let days = 22 + d + e;

    if d == 29 && e == 6 {
        EventDate::new(Event::holiday(String::from("Easter Monday"), Holiday::PublicHoliday, String::from("✝️") ), 19, 4)
    } else if d == 28 && e == 6 {
        EventDate::new(Event::holiday(String::from("Easter Monday"), Holiday::PublicHoliday, String::from("✝️") ), 18, 4)
    } else {
        if days > 31 {
            EventDate::new(Event::holiday(String::from("Easter Monday"), Holiday::PublicHoliday, String::from("✝️") ), (days-31) as u8, 4)
        } else {
            EventDate::new(Event::holiday(String::from("Easter Monday"), Holiday::PublicHoliday, String::from("✝️") ), days as u8, 3)
        }
    }
}

fn good_friday(y: u16) -> EventDate {
    let year = chrono::Utc::now().year() as u16; 
    let easter_mon = easter_monday(y);
    let mut good_friday_day = easter_mon.day - 2; // Subtract 2 days to get Good Friday

    // Adjust the month if the day is less than 1
    let mut good_friday_month = easter_mon.month;
    if good_friday_day < 1 {
        good_friday_day += 30; // Assuming March has 30 days for simplicity
        good_friday_month -= 1;
    }
    EventDate::new(Event::holiday(String::from("Good Friday"), Holiday::PublicHoliday, String::from("✝️") ), good_friday_day, good_friday_month)
}


