
use chrono::{Datelike, NaiveDate};
use crate::dates::holidays;
use crate::dates::structs::*;
use crate::display::utils::*;

pub fn show_month(month: &Month){

    print_title(&month);

    println!("   M   T   W   T   F   S   S");

    print_days(&month);
}

fn print_title(month: &Month) {
   let width: usize = 26;
   let title = String::from(" ")+&month.name + " " + &month.year.to_string() + " ";
   let left_length = (width - title.len()) / 2;
   let right_length = width - title.len() - left_length;

   let result = String::from("  ")+&"-".repeat(left_length) + &title + &"-".repeat(right_length);
   println!("{}", result);
}


fn print_days(month: &Month) {
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

    for d in 1..=days_in_month {
        let mut formatted_day_number = if d < 10 {format!("   {}",d)} else {format!("  {}", d)}; 
        
        let holidays = holidays::all_holidays();

        for h in holidays.iter() {
            if h.equals(d, month.number, month.year) {
                formatted_day_number = match h.holiday_type {
                    Holiday::Birthday => colour_birthday(&formatted_day_number).to_string(),
                    Holiday::PublicHoliday => colour_holiday(&formatted_day_number).to_string(),
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
                println!("{}", week);
                week = String::from("");
                first_week_printed = true;
                week_day_count = 0;
            }
        }

        else if week_day_count % 7== 0 || d == days_in_month {
            println!("{}", week);
            week = String::from("");
            week_day_count = 0;
        }
    }
}

pub fn month_as_vec(m: &Month) -> Vec<String> {
    let mut month: Vec<String> = vec![
        title(m),
        String::from("   M   T   W   T   F   S   S"),
    ];
    
    for week in get_weeks(m) {
        month.push(week);
    }

    month
}

fn title(month: &Month) -> String {
    let width: usize = 26;
   let title = String::from(" ")+&month.name + " " + &month.year.to_string() + " ";
   let left_length = (width - title.len()) / 2;
   let right_length = width - title.len() - left_length;

   let result = String::from("  ")+&"-".repeat(left_length) + &title + &"-".repeat(right_length);
   result
}


fn get_weeks(month: &Month) -> Vec<String> {
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
        let mut formatted_day_number = if d < 10 {format!("   {}",d)} else {format!("  {}", d)}; 
        
        let holidays = holidays::all_holidays();
        for h in holidays.iter() {
            if h.equals(d, month.number, month.year) {
                formatted_day_number = match h.holiday_type {
                    Holiday::Birthday => colour_birthday(&formatted_day_number).to_string(),
                    Holiday::PublicHoliday => colour_holiday(&formatted_day_number).to_string(),
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
