use serde::{Serialize, Deserialize};

use super::time24h::Time24h;



#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub start: Time24h,
    pub end: Time24h,
    pub icon: String
}


// #############################
//       IMPLEMENTATIONS
// #############################


impl Event {

    pub fn new(name: String, desc: String, start: Time24h, end: Time24h, icon: String) -> Self {
        if !is_emoji(&icon) { panic!("icon must be an emoji. (found {})", icon)}

        let icon_string = if icon == String::from(" ") { String::from("") } else { icon };
        Event {
            name: name,
            description: desc,
            start: start,
            end: end,
            icon: icon_string
        }
        
    }

    pub fn as_string(&self) -> String {

        let starts_at_midnight = self.start.hour == 0 && self.start.minute == 0;
        let ends_at_midnight = self.end.hour == 23 && self.end.minute == 59;
        if starts_at_midnight && ends_at_midnight {
            return format!("{}  (all day event)", self.description);
        }

        if self.start.equal(&self.end) {
            return format!("{} ({})", self.description, self.start.format());
        }

        format!("{} ({} --> {})",self.description, self.start.format(), self.end.format()) 
    }
}

fn is_emoji(s: &str) -> bool {
    s.chars().all(|c| {
        (c >= '\u{1F300}' && c <= '\u{1F6FF}') || // Miscellaneous Symbols and Pictographs
        (c >= '\u{1F900}' && c <= '\u{1F9FF}') || // Supplemental Symbols and Pictographs
        (c >= '\u{2600}' && c <= '\u{26FF}') // Miscellaneous Symbols
    })
}

