use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Time24h {
    pub hour: usize,
    pub minute: usize,
    pub second: Option<usize>
}


// #############################
//       IMPLEMENTATIONS
// #############################

impl Time24h {

    pub fn new(h: usize, m: usize) -> Self {
        if  (h > 24)  ||  (m > 60) {
            panic!("invalid time")
        } 
        Time24h {
            hour: h,
            minute: m,
            second: None
        }
    }

    /// midnight
    pub fn default() -> Self {
        Time24h {
            hour: 0,
            minute: 0,
            second: None 
        }
    }

    pub fn all_day() -> (Time24h, Time24h) {
        (Time24h::default(), Time24h{hour: 23, minute: 59, second: Some(59)})
    }    

}


impl Time24h {

    pub fn format(&self) -> String {

        let min: String = if self.minute < 10 { format!("0{}",self.minute) } else { self.minute.to_string() };
        let hr: String = if self.hour < 10 { format!("0{}", self.hour)} else { self.hour.to_string()};

        format!("{}:{}",hr, min)
    }

    pub fn equal(&self, other: &Time24h) -> bool {
        self.hour == other.hour && self.minute == other.minute
    }

}