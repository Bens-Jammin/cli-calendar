use crate::structures::holiday::ontario_public_holidays;
use crate::structures::{day::Day, month::Month, year::Year};
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