use std::fmt::format;

use chrono::{Datelike, Local, Timelike};

/// returns a progress bar representing the number of days passed in the year.
/// 
/// ### Example
/// ```
/// println!("{}", day_progress());
/// > [#####--------------------] (85/366)
/// ```
pub fn year_progress(enable_fraction: bool) -> String {
   let local_date = Local::now();
    let year = local_date.year();
    let days_passed = local_date.ordinal() as f64;

    // Get the total number of days in the current year
    let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366.0 } else { 365.0 };

    progress_bar(days_passed as f32, days_in_year as f32, enable_fraction)
}


/// returns a progress bar representing the number of days passed in the month.
/// 
/// ### Example
/// ```
/// println!("{}", day_progress());
/// > [####################-----] (25/31)
/// ```
pub fn month_progress(enable_fraction: bool) -> String {
    let local_date = Local::now();
    let month = local_date.month();
    let days_passed = local_date.day() as f64;

    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31.0,  // January, March, May, July, August, October, December
        4 | 6 | 9 | 11 => 30.0,                 // April, June, September, November
        2 => {
            if local_date.year() % 4 == 0 && (local_date.year() % 100 != 0 || local_date.year() % 400 == 0) {
                29.0 // Leap year
            } else {
                28.0 // Non-leap year
            }
        }
        _ => panic!("Invalid month"),
    };

    progress_bar(days_passed as f32, days_in_month as f32, enable_fraction)
}


/// returns a progress bar representing the number of minutes passed in the day.
/// 
/// ### Example
/// ```
/// println!("{}", day_progress());
/// >  [###########--------------] 44%  (634/1440)
/// ```
pub fn day_progress(enable_fraction: bool) -> String {

    let local_time = Local::now().time();
    let seconds_passed = (local_time.hour() * 3600) + (local_time.minute() * 60) + local_time.second() as u32;
    let total_seconds_in_day = 24 * 3600;

    progress_bar((seconds_passed as f32)/60.0, (total_seconds_in_day as f32)/60.0, enable_fraction )
}



fn progress_bar(curr: f32, total: f32, enable_fraction: bool) -> String{

    let progress_bar_width = 25; 

    let progress:f32 = curr/total;

    let progress_ammount = (progress * progress_bar_width as f32) as usize;
    let empty_amount = progress_bar_width - progress_ammount;

    // todo: eventually add this to settings
    let progress_text = format!("\x1b[34m{}\x1b[0m", "#".repeat(progress_ammount));
    let empty_text = "-".repeat(empty_amount);
    
    if enable_fraction {
        format!("[{}{}] {}%  ({}/{})", progress_text, empty_text,(curr * 100.0/total) as usize, curr as usize, total as usize )
    } else {
        format!("[{}{}] {}%", progress_text, empty_text,(curr * 100.0/total) as usize)
    }

//    String::from("This is a temp return. You shouldn't be able to read this. ( i'm in src/display/progress_bar.rs )")
}