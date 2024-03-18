use chrono::{Datelike, Local, NaiveDate};

use crate::dates::structs::*;


impl Day {
    pub fn new(day: usize, month: usize, year: usize, h_type: Holiday) -> Day {
        Day {
            num: day,
            month: month,
            year_no: year,
            day_name: Day::calculate_day_name(day as u32,month as u32, year as i32),
            events: vec![],
            holiday_type: h_type,
        }
    }

    pub fn add(&mut self, event: Event) { self.events.push(event); }
    pub fn remove(&mut self, idx: usize) { self.events.remove(idx); }

    pub fn print_events(&self) {
        for e in &self.events {
            let today = chrono::Utc::now().day() as usize;
            let mut day_label = match (self.num - today) as isize {
                0 =>  String::from("today"),
                1 =>  String::from("tomorrow"),
                -1 => String::from("yesterday"),
                _ =>  String::from(&self.day_name)
            };
            let padding_size: usize = 8;
            let day_padding = " ".repeat( padding_size - day_label.len() );
            day_label += &(day_padding + &Month::name_from_month_number(self.month) +" "+&(self.num.to_string())+" "+ &self.year_no.to_string());
            println!("{}  {}",day_label, e.as_string());
            
        }
    }

    pub fn holiday(day: usize, month: usize, year: usize, name: String, h_type: Holiday) -> Day {
        Day {
            holiday_type: h_type, 
            num: day,
            month: month,
            year_no: year,
            day_name: Day::calculate_day_name(day as u32,month as u32, year as i32),
            events: vec![
                Event{ 
                    name: String::from("Holiday"), 
                    description: name, 
                    start: Time24h::new(0, 0) 
                }
            ]
        }
    }

    pub fn equals_day(&self, other: Day) -> bool {
           self.num == other.num 
        && self.month == other.month
        && self.year_no == other.year_no
    }

    pub fn equals(&self, d: usize, m: usize, y: usize) -> bool {
           self.num == d 
        && self.month == m
        && self.year_no == y
    }

    pub fn today() -> Day {
        let today = Local::now();
        Day::new(
            today.day() as usize, 
            today.month() as usize, 
            today.year() as usize, 
            Holiday::None
        )
    }
}

impl Day {
    fn calculate_day_name(day: u32, month: u32, year: i32) -> String {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("Error parsing date");
        date.weekday().to_string()
    }
}