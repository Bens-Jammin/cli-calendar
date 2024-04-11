use chrono::Weekday::*;
use chrono::Datelike;
use crate::structures::month::Month;


pub fn show_upcoming_events(month: &Month, next_month: &Month) {
    let today = chrono::Utc::now().weekday();
    let day_num = chrono::Utc::now().day();
    let m = Month::name_from_month_number(chrono::Utc::now().month().try_into().unwrap());
    let y = chrono::Utc::now().year();

    let date = match today {
        Mon => "Monday, ",
        Tue =>  "Tuesday, ",
        Wed =>  "Wednesday, ",
        Thu =>  "Thursday, ",
        Fri =>  "Friday, ",
        Sat =>  "Saturday, ",
        Sun =>  "Sunday, ",
    };

    let formatted_date = format!("{}{} {}, {}", date, m, day_num, y);
    println!("\nToday is {}.\n\nUpcoming events this month:\n",formatted_date);
    // todo: should go to the next month too if possible (make it a second arg?)
    
    for d in &month.days {
        if (day_num..=day_num + 7).contains(&(d.num as u32)) {
            d.print_events();
        }
    }

   for d in &next_month.days {
        
        if (day_num..=day_num + 7 - month.days.len() as u32 ).contains(&(d.num as u32)) {
            d.print_events();
        }else{
            break;
        } 
    } 
}

/// returns a vector of formatted events coming in the next week. 
/// 
/// 
/// ### Note
/// The first four elements are set, and are as follows:
/// 
/// 0. empty string
/// 1. today's date
/// 2. empty string
/// 3. a bold string containing "Upcoming events:" 
/// 
/// ### Example
/// ```
/// [
///     "Monday, March 25, 2024"
///     "Today      March 25 2024   Coffee date (11:00 --> 12:00)"
///     "Thursday   March 28 2024   professional practice Midterm (19:30 --> 21:00)"
///     "Friday     March 29 2024   Good Friday (all day event)"
///     ...
/// ]
/// ```
pub fn get_incoming_events(month: &Month, next_month: &Month) -> Vec<String> {
    let mut events: Vec<String> = Vec::new();

    let today = chrono::Utc::now().weekday();
    let day_num = chrono::Utc::now().day();
    let m = Month::name_from_month_number(chrono::Utc::now().month().try_into().unwrap());
    let y = chrono::Utc::now().year();

    let date = match today {
        Mon => "Monday, ",
        Tue =>  "Tuesday, ",
        Wed =>  "Wednesday, ",
        Thu =>  "Thursday, ",
        Fri =>  "Friday, ",
        Sat =>  "Saturday, ",
        Sun =>  "Sunday, ",
    };

    let formatted_date = format!("Today is {}{} {}, {}.", date, m, day_num, y);
    events.extend( vec![
            String::from(""),
            formatted_date,
            String::from(""),
            String::from("\x1b[4mUpcoming events:\x1b[0m")
        ]);
    
    for d in &month.days {
        if (day_num..=day_num + 7).contains(&(d.num as u32)) {
            let day_events = d.events();
            events.extend(day_events);
        }
    }

    // no point in checking the next month if its >1 week away
    if day_num + 7 < month.days.len() as u32 { return events; }

    for d in &next_month.days {
        if (0..=day_num + 7 - month.days.len() as u32 ).contains(&(d.num as u32)) {
            let day_events = d.events();
            events.extend(day_events);
        }else{
            break;
        } 
    }
    events
}
    

