use serde::{Serialize, Deserialize};
use format::emoji::is_valid_emoji;
use crate::format;
use super::time24h::Time24h;



#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub start: Time24h,
    pub end: Time24h,
    pub icon: String,
    pub priority: bool
}


// #############################
//       IMPLEMENTATIONS
// #############################


impl Event {

    pub fn new(name: String, desc: String, start: Time24h, end: Time24h, icon: String, priority:bool) -> Self {


        if !is_valid_emoji(&icon) { panic!("icon must be an emoji. (found {:?})", icon)}

        let icon_string = if icon == String::from(" ") { String::from("") } else { icon };
        Event {
            name: name,
            description: desc,
            start: start,
            end: end,
            icon: icon_string,
            priority: priority
        }
        
    }

    pub fn as_string(&self) -> String {

        

        if self.start.equal(&self.end) {
            return format!("{}", self.description,);
        }

        let priority = if self.priority {
            format!("\x1B[91m!!!\x1B[0m")
        } else { format!("") };

        format!("{}{}", priority, self.description) 
    }


    pub fn timespan(&self) -> String {
        
        let starts_at_midnight = self.start.hour == 0 && self.start.minute == 0;
        let ends_at_midnight = self.end.hour == 23 && self.end.minute == 59;
        if starts_at_midnight && ends_at_midnight {
            return String::from("(all day)");
        }
        
        format!("({} --> {})", self.start.format(), self.end.format())
    }

}

fn is_emoji(s: &str) -> bool {
    s.chars().all(|c| {
        (c >= '\u{1F300}' && c <= '\u{1F6FF}') || // Miscellaneous Symbols and Pictographs
        (c >= '\u{1F900}' && c <= '\u{1F9FF}') || // Supplemental Symbols and Pictographs
        (c >= '\u{2600}' && c <= '\u{26FF}') // Miscellaneous Symbols
    })
}

