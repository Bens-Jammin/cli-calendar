use chrono::Datelike;

use crate::dates::structs::*;

impl Year {
    pub fn new() -> Year {
        let current_year = chrono::Utc::now().year() as usize;
        Year {
            year: current_year,
            months: [
                Month::new(1, current_year),
                Month::new(2, current_year),
                Month::new(3, current_year),
                Month::new(4, current_year),
                Month::new(5, current_year),
                Month::new(6, current_year),
                Month::new(7, current_year),
                Month::new(8, current_year),
                Month::new(9, current_year),
                Month::new(10, current_year),
                Month::new(11, current_year),
                Month::new(12, current_year)
            ]    
        }
    }
}