use chrono::Datelike;
use serde::{Serialize, Deserialize};

use super::{day::Day, event::Event, holiday::all_holidays, month::Month};

#[derive(Serialize, Deserialize)]
pub struct Year {
    pub year: u16,
    pub months: [Month; 12]
}


// #############################
//       IMPLEMENTATIONS
// #############################

impl Year {
    pub fn new() -> Year {
        let current_year = chrono::Utc::now().year() as u16;
        let mut year = Year {
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
                Month::new(12, current_year),
            ]
        };
        year.add_holidays();
        year
    }
}


// ######################################
//     ADDING / INSTANCE MANIPULATION
// ######################################


impl Year {

    /// adds an event to the given day
    /// 
    /// ### NOTE
    /// 
    /// day and month numbers must start from 0.
    /// 
    /// 
    /// ### Example
    /// 
    /// ```
    /// let year = Year::new();
    /// let event = Event{ ... }
    /// year.add_event(30, 0, &event);
    /// ```
    pub fn add_event(&mut self, d: u8, m: u8, e: &Event) {
        if m > 12 { panic!("month is the second parameter. The month number should be in the inclusive range [1, 12] ") }
        if d > 31 { panic!("The day number should be in the inclusive range [1, 31].") }

        let day = self.month(m as usize -1).day(d as usize -1);
        let mut updated_day = day.duplicate();
        updated_day.add_event(e);
        self.months[m as usize -1].update_day(d as usize -1, updated_day);
    }


    /// repeats the given event so that it occurs on the  ```day```th day every month 
    /// between ```start_month``` and ```start_month + repeat_freq```
    /// 
    /// ### Example
    /// For an event to occur on the 6th day in the months March, April, May, June:
    /// ```
    /// let mut year = Year { ... }
    /// let event = Event { ... }
    /// year.repeat_monthly(&e, 6, 2, 4)
    /// ```
    pub fn repeat_monthly(&mut self, e: &Event, day: u8, start_month: u8, repeat_freq: u8) {
        for m in start_month..(start_month+repeat_freq) {
            self.add_event(day, m, e);
        }
    }


    pub fn repeat_weekly(e: &Event, start_day: u8, start_month: u8, repeat_freq: u8) {
        
        todo!("Not implemented yet");       
    } 
    

    pub fn repeat_daily(e: &Event, start_day: u8, start_month: u8, repeat_freq: u8) {
        todo!("Not implemented yet!");    
    }


    fn add_holidays(&mut self) {
        for h in all_holidays() {
            self.set_date(h);
        }
    }


    pub fn set_date(&mut self, day: Day) {
        if let Some(month) = self.months.get_mut((day.month - 1) as usize) {
            if let Some(day_to_update) = month.days.get_mut(day.num - 1) {
                *day_to_update = day;
            } else {
                println!("Day {} does not exist in month number {} (days vector in month has {} elements)", day.num, day.month, month.days.len());
            }
        } else {
            println!("Month number {} does not exist", day.month);
        }
    }


    pub fn clear_all(&mut self) {
        for month in &mut self.months {
            month.clear_events();
        }
        self.add_holidays();
    }

}



// ###############
//     GETTERS
// ###############


impl Year {

    pub fn month(&self, m: usize) -> &Month {
        &self.months[m]
    }

    pub fn current_month(&self) -> &Month {
        let m = chrono::Utc::now().month();
        &self.month((m-1) as usize)
    }

    pub fn next_month(&self) -> &Month {
        let m = chrono::Utc::now().month();
        &self.month(m as usize)
    }

}