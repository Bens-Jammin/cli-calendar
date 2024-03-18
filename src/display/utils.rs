use ansi_colors::*;

use crate::{display::*, dates::structs::Month};
use std::fmt;

pub fn available_highlights() -> [String; 4] { 
    [
        colour_birthday("Birthdays"), 
        colour_event("Events"), 
        colour_holiday("Holidays"), 
        colour_today("Today")
    ]
}

pub fn display_calendar(month: &Month) {
    println!("\n");
    calendar::show_month(month);
    println!("\n");
    incoming_events::show_upcoming_events(month);
}

pub fn display_calendar_with_highlighting( month: &Month) {
    
    let calendar_items = calendar::month_as_vec(month);
    println!("\n");
    
    for i in 0..calendar_items.len() {
        let highlights = available_highlights();
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


pub fn colour_birthday(txt: &str) -> String {
    format!("\x1b[1;34m{}\x1b[0m", txt) // Blue color
}

pub fn colour_event(txt: &str) -> String {
    format!("\x1b[1;32m{}\x1b[0m", txt) // Green color
}

pub fn colour_holiday(txt: &str) -> String {
    format!("\x1b[1;35m{}\x1b[0m", txt) // Bold pink color
}

pub fn colour_today(txt: &str) -> String {
    format!("\x1b[1;5m{}\x1b[0m", txt) // Bold blinking text
}
