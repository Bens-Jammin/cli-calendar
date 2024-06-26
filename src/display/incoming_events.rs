use std::fmt::format;

use chrono::Weekday::*;
use chrono::Datelike;
use crate::structures::month::Month;


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
///     "Today      March 25 2024   Coffee date (11:00 --> 12:00)"
///     "Thursday   March 28 2024   professional practice Midterm (19:30 --> 21:00)"
///     "Friday     March 29 2024   Good Friday (all day event)"
///     ...
/// ]
/// ```
pub fn get_incoming_events(month: &Month, next_month: &Month) -> Vec<String> {

    let todays_number = chrono::Utc::now().day();
    let mut upcoming_events = vec![
        String::from("Upcoming events this week:"),
        String::new()
    ];
    for d in todays_number..=todays_number+7 {
        if d > month.days.len() as u32 { break; }

        if month.day(d as usize).events().len() == 0 { continue; }
        let day = month.day(d as usize);
        upcoming_events.push(day.title());

        for e in day.events.iter() {
            upcoming_events.push( format!(" ↳{}", e.as_string()) );
            upcoming_events.push( format!(" time: {}", e.timespan()) );
        }
        upcoming_events.push(String::new());
    }

    upcoming_events

    // REDO THE WHOLE FUCKING THING !!!
    // it should show the date, a little |-> arrow then the items for every event that day
    // 
    // 
    // ex:
    // Today, April 11
    //     |--> event 1 strin
    //     |--> Event 2 string
    // Tomorrow, April 12
    //     |--> Event 3 string
    // etc etc

}
    

