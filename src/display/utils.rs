use crate::structures::holiday::ontario_public_holidays;
use crate::structures::{holiday::holiday_hash_map, day::Day, month::Month, year::Year};
use crate::display::*;
use self::colours::*;

pub fn available_highlights() -> [String; 5] {
    [   
        String::from("Birthdays"),
        String::from("Events"), 
        String::from("Holidays"), 
        String::from("Today"),
        String::from("Vacations")
    ]
}

pub fn highlighted_highlights() -> [String; 5] { 
    [
        colour_birthday("Birthdays"), 
        colour_event("Events"), 
        colour_holiday("Holidays"), 
        colour_today("Today"),
        colour_vacations("Vacations")
    ]
}


pub fn display_calendar(year: &Year, m: usize) {
    println!("\n");
    calendar::month::show_3month_window(year, m);
    println!("\n");
    incoming_events::show_upcoming_events(year.month(m), year.month(m+1));
}


pub fn display_calendar_with_highlighting( month: &Month) {
    
    let calendar_items = calendar::month::month_as_vec(month);
    println!("\n");
    
    for i in 0..calendar_items.len() {
        let highlights = highlighted_highlights();
        let legend = if i == 0 {
                "-- Legend --"
            } else if (i-1) >= highlights.len() {
                ""
            } else {
                &highlights[i-1] 
            };

        println!("{}     {}",calendar_items.get(i).unwrap(), legend);
    }
}


pub fn print_legend() {
    for r in highlight_legend() {
        println!("{}", r);
    }
}


pub fn highlight_legend() -> Vec<String> {
    let mut legend = highlighted_highlights().to_vec();
    legend.insert(0, String::from("\x1b[1m-- Legend --\x1b[0m") );
    legend
}


pub fn display_holiday(day: &Day) -> String {

    let hashmap = holiday_hash_map();
    let result = String::from("");
    for h in ontario_public_holidays() {
        if h.equals_day(day) {
            if let Some(icon) = hashmap.get(&h) {
                return icon.to_string();
            } else {
                return if day.num > 10 { day.num.to_string() }  else { format!("0{}",day.num) }
            }
        }
    }
    result
}