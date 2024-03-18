use crate::dates::structs::*;


impl Event {
    pub fn as_string(&self) -> String {
        format!("{} at {}",self.name, self.format_time()) 
    }

    pub fn format_time(&self) -> String {
        self.start.format()
    }
}