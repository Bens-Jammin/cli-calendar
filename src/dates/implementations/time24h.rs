use crate::dates::structs::*;


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

    pub fn format(&self) -> String {
        let min: String = if self.minute < 10 { format!("0{}",self.minute) } else { self.minute.to_string() };
        let hr: String = if self.hour < 10 { format!("0{}", self.hour)} else { self.hour.to_string()};

        format!("{}:{}",hr, min)
    }
}