use chrono::{Datelike, Local, NaiveDate };
use indicatif::*;
use crate::{structures::{day::Day, holiday::{self, Holiday}, month::Month}, display::{colours::*, utils}};

#[warn(unused_features)]
pub fn year_progress_bar() {
    let year = chrono::Utc::now().year();
    let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
    let days_passed = Local::now().ordinal() - 1;
    let progress_percent = (days_passed as f64 / days_in_year as f64) * 100.0;

    let progress_bar = ProgressBar::new(days_in_year as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{bar:50.cyan/blue}] ({pos}/{len})")
        .unwrap()
        .progress_chars("##-"));

    println!("Year Progress: {:.2}%   ", progress_percent);
    progress_bar.set_position(days_passed as u64);
    progress_bar.finish();
}



/// returns the month in vector format, formatted with colours and emojis
/// each element in the vector is either the month title or a week 
pub fn get_weeks_formatted(month: &Month) -> Vec<String> {
    let first_day_of_month = NaiveDate::from_ymd_opt(
        month.year as i32,
        month.number as u32, 
        1
    ).expect("An error occured while parsing date");

    let first_week_offset = first_day_of_month.weekday().num_days_from_monday() as usize;

    let mut week = "    ".repeat(first_week_offset);
    let days_in_month = Month::num_days(month.number, month.year); 

    let mut first_week_printed: bool = false;
    let mut week_day_count = 0;

    let mut weeks: Vec<String> = vec![];

    for d in 1..=days_in_month {

        let day = month.days.get(d-1).unwrap();


       let day_icon = day.icon();

        // I want monday to have less left padding so that it fits into boxes better
        // i.e. it centers the calendar in a content box better
        let date_left_padding = if week_day_count == 0 { String::from("") } else { String::from("  ") };
        let mut formatted_day_number = if day_icon.len() == 0 {
            if d < 10 {
                format!("{}0{}",date_left_padding, d)
            } else { 
                format!("{}{}",date_left_padding, d) 
            }
        } else { day_icon };
        
        if day.has_non_special_events() {
            formatted_day_number = colour_event(&formatted_day_number);
        }

        // set colour for the 'day number'
        let holidays = holiday::all_holidays();
        for h in holidays.iter() {
            if h.equals(d, month.number, month.year) {
                formatted_day_number = match h.holiday_type {
                    Holiday::Birthday => colour_birthday(&formatted_day_number).to_string(),
                    Holiday::PublicHoliday => colour_holiday(&formatted_day_number).to_string(),
                    Holiday::Vacation => colour_vacations(&formatted_day_number).to_string(),
                    Holiday::None => continue,
                };
            }
        }

        if Day::today().equals(d, month.number, month.year) {
            formatted_day_number = colour_today(&formatted_day_number).to_string();
        } 

        week = String::from(week) + &formatted_day_number; 
        week_day_count += 1;

        
        if !first_week_printed {
            if d % (7 - first_week_offset) == 0 {
                weeks.push(String::from(week));
                week = String::from("");
                first_week_printed = true;
                week_day_count = 0;
            }
        }

        else if week_day_count % 7== 0 || d == days_in_month {
            weeks.push(String::from(week));
            week = String::from("");
            week_day_count = 0;
        }
    }
    weeks
}


/// returns the month in vector form without formatting (colouring, etc)
pub fn get_weeks_raw(month: &Month) -> Vec<String> {
    let first_day_of_month = NaiveDate::from_ymd_opt(
        month.year as i32,
        month.number as u32, 
        1
    ).expect("An error occured while parsing date");

    let first_week_offset = first_day_of_month.weekday().num_days_from_monday() as usize;

    let mut week = "    ".repeat(first_week_offset);
    let days_in_month = Month::num_days(month.number, month.year); 

    let mut first_week_printed: bool = false;
    let mut week_day_count = 0;

    let mut weeks: Vec<String> = vec![];

    for d in 1..=days_in_month {
        
        let formatted_day_number = if d < 10 {format!("   {}",d)} else {format!("  {}", d)}; 
        
        week = String::from(week) + &formatted_day_number; 
        week_day_count += 1;

        if !first_week_printed {
            if d % (7 - first_week_offset) == 0 {
                weeks.push(String::from(week));
                week = String::from("");
                first_week_printed = true;
                week_day_count = 0;
            }
        }

        else if week_day_count % 7== 0 || d == days_in_month {
            weeks.push(String::from(week));
            week = String::from("");
            week_day_count = 0;
        }
    }
    weeks
}