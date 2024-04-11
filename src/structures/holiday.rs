use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{Datelike, NaiveDate, Weekday};

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



pub fn all_holidays() -> Vec<Day> {
    let mut holidays = birthdays();
    let pub_holidays = ontario_public_holidays();
    holidays.extend( pub_holidays );
    holidays
}

pub fn birthdays() -> Vec<Day> {
    let year = chrono::Utc::now().year() as u16; 
    vec![
        Day::holiday(9,9, year, String::from("My Birthday"), Holiday::Birthday),
        Day::holiday(13,11,year, String::from("Tyler Heinz's Birthday"), Holiday::Birthday),
        Day::holiday(24, 3, year, String::from("Lauren Wheatley's Birthday"), Holiday::Birthday),
        Day::holiday(4,10,year, String::from("Kaydence White's Birthday"), Holiday::Birthday)
    ]
}


pub fn ontario_public_holidays() -> Vec<Day> {
    let year = chrono::Utc::now().year() as u16; 
    let y = year as i32;
    vec![
        Day::holiday(1               , 1, year, String::from("New Years Day"), Holiday::PublicHoliday),
        Day::holiday(family_day(y)   , 2, year, String::from("Family Day"), Holiday::PublicHoliday),
        Day::holiday(14              , 2, year, String::from("Valentine's Day"), Holiday::PublicHoliday),
        good_friday(year),
        easter_monday(year),
        Day::holiday(1                , 4, year, String::from("April Fool's Day"), Holiday::PublicHoliday),
        Day::holiday(victoria_day(y)  , 5, year, String::from("Victoria Day"), Holiday::PublicHoliday),
        Day::holiday(1                , 7, year, String::from("Canada Day"), Holiday::PublicHoliday),
        Day::holiday(civic_holiday(y) , 8, year, String::from("Civic Holiday"), Holiday::PublicHoliday),
        Day::holiday(labour_day(y)    , 9, year, String::from("Labour Day"), Holiday::PublicHoliday),
        Day::holiday(30               , 9, year, String::from("Truth and Reconciliation Day"), Holiday::PublicHoliday),
        Day::holiday(thanksgiving(y)  , 10, year, String::from("Thanksgiving"), Holiday::PublicHoliday),
        Day::holiday(31               , 10, year, String::from("Halloween"), Holiday::PublicHoliday),
        Day::holiday(11               , 11, year, String::from("Rememberance Day"), Holiday::PublicHoliday),
        Day::holiday(25               , 12, year, String::from("Christmas"), Holiday::PublicHoliday),
        Day::holiday(31               , 12, year, String::from("New Years Eve"), Holiday::PublicHoliday), 
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
pub fn easter_monday(y: u16) -> Day {
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
        Day::holiday(19, 4, y, String::from("Easter Monday"), Holiday::PublicHoliday)
    } else if d == 28 && e == 6 {
        Day::holiday(18, 4, y, String::from("Easter Monday"), Holiday::PublicHoliday)
    } else {
        if days > 31 {
            Day::holiday((days-31) as u8, 4, y, String::from("Easter Monday"), Holiday::PublicHoliday)
        } else {
            Day::holiday(days as u8, 3, y, String::from("Easter Monday"), Holiday::PublicHoliday)
        }
    }
}

fn good_friday(y: u16) -> Day {
    let year = chrono::Utc::now().year() as u16; 
    let easter_mon = easter_monday(y);
    let mut good_friday_day = easter_mon.num - 2; // Subtract 2 days to get Good Friday

    // Adjust the month if the day is less than 1
    let mut good_friday_month = easter_mon.month;
    if good_friday_day < 1 {
        good_friday_day += 30; // Assuming March has 30 days for simplicity
        good_friday_month -= 1;
    }
    Day::holiday(good_friday_day, good_friday_month, year,String::from("Good Friday"), Holiday::PublicHoliday)
}


// NOTE: soon to be obsolete. Events now have an icon attribute.
pub fn holiday_hash_map() -> HashMap<Day, String> {
    let mut hashmap = HashMap::new();

    let holidays = ontario_public_holidays();
    hashmap.insert(holidays[0].duplicate(), String::from("🎉"));
    hashmap.insert(holidays[2].duplicate(), String::from("❤️"));
    hashmap.insert(holidays[3].duplicate(), String::from("✝️"));
    hashmap.insert(holidays[4].duplicate(), String::from("✝️"));
    hashmap.insert(holidays[9].duplicate(), String::from("⚒️"));
    hashmap.insert(holidays[11].duplicate(), String::from("🦃"));
    hashmap.insert(holidays[12].duplicate(), String::from("🎃"));
    hashmap.insert(holidays[14].duplicate(), String::from("🎄"));
    hashmap.insert(holidays[15].duplicate(), String::from("🎉"));
    
    hashmap
}