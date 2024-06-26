use std::hash::Hash;
use std::vec;

use chrono::{Datelike, Local, NaiveDate, Utc};
use serde::{Serialize, Deserialize};

use crate::{display::colours::*, structures::month::Month};
use crate::structures::{event::Event, holiday::Holiday};

use super::event::{self, EventType};
use super::holiday::{self, ontario_public_holidays};
use super::time24h::Time24h;


#[derive(Serialize, Deserialize, Clone)]
pub struct Day {
    pub num: u8,
    pub month: u8,
    pub year_no: u16,
    pub day_name: String,
    pub events: Vec<Event>,
    pub holiday_type: Holiday
}


// #############################
//       IMPLEMENTATIONS
// #############################


impl Day {

    pub fn new(day: u8, month: u8, year: u16, h_type: Holiday) -> Day {
        Day {
            num: day,
            month: month,
            year_no: year,
            day_name: Day::calculate_day_name(day as u32, month as u32, year as i32),
            events: vec![],
            holiday_type: h_type,
        }
    }

    pub fn holiday(day: u8, month: u8, year: u16, name: String, h_type: Holiday, holiday_icon: String) -> Day {
        
        let event_name = match h_type {
            Holiday::Birthday => "Birthday",
            Holiday::PublicHoliday => "Public Holiday",
            Holiday::Vacation => "Vacation",
            Holiday::None => panic!("Holiday must have a valid Holiday enum value (NOT NONE !!!)."),
        };
        let (s, e) = Time24h::all_day();

        let event_type = match h_type {
            Holiday::Birthday => EventType::Birthday,
            Holiday::PublicHoliday => EventType::PublicHoliday,
            Holiday::Vacation => EventType::Vacation,
            Holiday::None => EventType::Event,
        };

        Day {
            holiday_type: h_type, 
            num: day,
            month: month,
            year_no: year,
            day_name: Day::calculate_day_name(day as u32,month as u32, year as i32),
            events: vec![
                Event{ 
                    name: String::from(event_name), 
                    description: name, 
                    start: s,
                    end : e,
                    icon: holiday_icon,
                    priority: false,
                    event_type: event_type
                }
            ]
        }
    }

    /// creates an instance of Day with the current date
    pub fn today() -> Day {
        let today = Local::now();
        Day::new(
            today.day() as u8, 
            today.month() as u8, 
            today.year() as u16, 
            Holiday::None
        )
    }

    pub fn duplicate(&self) -> Day {
        Day {
            num: self.num,
            month: self.month,
            year_no: self.year_no,
            day_name: (*self.day_name).to_string(),
            events: (*self.events).to_vec(),
            holiday_type: self.holiday_type,
        }
    }

    pub fn null() -> Day {
        Day {
            num: 0,
            month: 0,
            year_no: 0,
            day_name: String::new(),
            events: Vec::new(),
            holiday_type: Holiday::None
        }

    }

}


// ######################################
//     ADDING / INSTANCE MANIPULATION
// ######################################


impl Day {

    pub fn add_event(&mut self, e: &Event) {
        self.events.push(e.clone());
    }

/// clears all non holiday and all non birthday events
pub fn clear_events(&mut self) {
    self.events = vec![];
}


pub fn remove_event(&mut self, idx: usize) {
    self.events.remove(idx);
}


}


// ###############
//     GETTERS 
// ###############

impl Day {

    /// returns the icon representation of the date.
    /// The icon to use is the first event in the vector that has an icon
    ///
    /// **PRIRORITY FOR DATE FORMATTING** (highest --> lowest)
    /// 
    /// * today
    /// * special holiday (represented by an emoji)
    /// * non-special holiday (birthday, vacation, holidays like labour day ,etc)
    /// * any other events
    ///
    /// if the icon is an empty string (i.e. '') then it just shows the date
    pub fn icon(&self) -> String {
        let mut icon = String::from("");

        for e in &self.events {
            if e.icon.len() != 0 {
                icon = format!("{}", e.icon.to_string());
            }
        }

        icon
    }


    pub fn formatted_date(&self) -> String {
        
        if self.icon() != "" {
            return self.icon();
        }

        let date_number: String;

        if self.num < 10 {
            date_number = format!("0{}", self.num);
        } else {
            date_number = format!("{}", self.num)
        }

        self.colour(date_number)
    }
}   


// #############################
//        MISC / DISPLAY
// #############################

impl Day {

    pub fn equals_day(&self, other: &Day) -> bool {
           self.num == other.num 
        && self.month == other.month
        && self.year_no == other.year_no
    }

    pub fn equals(&self, d: u8, m: u8, y: u16) -> bool {
           self.num == d 
        && self.month == m
        && self.year_no == y
    }

    
    pub fn events(&self) -> Vec<String> {
        let mut events: Vec<String> = vec![];
        for e in &self.events {
            let today = chrono::Utc::now().day() as usize;
            let mut day_label = match self.num as isize - today as isize {
                0 =>  colour_today("Today"),
                1 =>  colour_tomorrow("Tomorrow"),
                _ =>  String::from(&self.day_name)
            };

            let padding_size: usize = 9 ;
            let day_padding = if day_label.len() > padding_size { String::from("") } else {" ".repeat( padding_size - day_label.len() )};
            day_label += &(day_padding + &Month::name_from_month_number(self.month) +" "+&(self.num.to_string())+" "+ &self.year_no.to_string());
            events.push(format!("{} |",day_label));
            events.push(format!("   {}",e.as_string()));
        }
        events
    }


    fn colour(&self, text: String) -> String {

        if Day::today().equals(self.num as u8, self.month, self.year_no) {
            return colour_today(&text);
        } 

        // set colour for the 'day number'
        let holidays = holiday::all_holidays();
        let mut formatted_text = String::from(&text);
        for h in holidays.iter() {
            if self.equals(h.day, h.month, self.year_no) {
                formatted_text = match h.event.event_type {
                    EventType::Birthday => colour_birthday(&formatted_text).to_string(),
                    EventType::PublicHoliday => colour_holiday(&formatted_text).to_string(),
                    EventType::Vacation => colour_vacations(&formatted_text).to_string(),
                    EventType::Event => continue,
                };
            }
        }
        if formatted_text != text { return formatted_text; }

        if self.events.len() > 0 {
            return colour_event(&text);
        }
        text
    }

    
    /// returns true if the day has a 'standard' event
    /// 
    /// ### Note
    /// a non-standard event is an event which is **not** one of the following:
    /// - a Birthday
    /// - a public holiday
    /// 
    /// ### Example
    /// ```
    /// let d = Day{ ... events: vec![birthday, appointment] }
    /// d.has_non_special_events();
    /// > true
    /// 
    /// let d2 = Day{ ... events: vec![]}
    /// d2.has_non_special_events();
    /// > false
    /// 
    /// let d3 = Day{ ... events: vec![birthday, halloween]}
    /// d3.has_non_special_events();
    /// > false
    /// ```
    pub fn has_non_special_events(&self) -> bool {
        self.events.len() > 0 && self.holiday_type == Holiday::None
    }


    fn calculate_day_name(day: u32, month: u32, year: i32) -> String {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("Error parsing date");
        date.weekday().to_string()
    }


    /// returns the 'title' of the day, to be used for upcoming events.
    /// 
    /// The title contains the name (i.e. Monday, Tuesday, ...) unless that day is today, and the day and month
    /// 
    /// ### Example
    /// Today   Apr 11
    /// Friday  Apr 12
    /// ... 
    pub fn title(&self) -> String {
        let max_name_length = 7;

        let mut title = format!("{}{}",self.day_name, " ".repeat(max_name_length - self.day_name.len()) );

        if self.equals_day(&Day::today()) {
            title = format!("Today{}"," ".repeat(max_name_length - 5));
        }

        if self.num < 10 {
            return format!("{} {} 0{}",title, Month::name_from_month_number(self.month), self.num) 
    
        }
        format!("{} {} {}",title, Month::name_from_month_number(self.month), self.num)
    }

}


// #################################
//       TRAIT IMPLEMENTATIONS
// #################################


impl Hash for Day {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.num.hash(state);
        self.month.hash(state);
        self.year_no.hash(state);
        self.day_name.hash(state);
        self.events.hash(state);
        self.holiday_type.hash(state);
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.year_no == other.year_no 
        && self.month == other.month 
        && self.num == other.num
    }
}

impl Eq for Day {}