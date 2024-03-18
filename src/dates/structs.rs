use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Year {
    pub year: usize,
    pub months: [Month; 12]
}

#[derive(Serialize, Deserialize)]
pub struct Month {
    pub number: usize,
    pub year: usize,
    pub name: String,
    pub days: Vec<Day>
}

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub num: usize,
    pub month: usize,
    pub year_no: usize,
    pub day_name: String,
    pub events: Vec<Event>,
    pub holiday_type: Holiday
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub start: Time24h
}

#[derive(Serialize, Deserialize)]
pub struct Time24h {
    pub hour: usize,
    pub minute: usize,
    pub second: Option<usize>
}

#[derive(Serialize, Deserialize)]
pub enum Holiday {
    Birthday,
    PublicHoliday,
    None
}