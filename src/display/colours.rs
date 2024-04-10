use std::collections::HashMap;

use crate::data_io::{paths::settings_file_path, parser::parse_colour_settings};



pub fn colour_birthday(txt: &str) -> String {
    let data = parse_colour_settings(&settings_file_path());
    let c = format!("{}{}\x1B[0m",convert_to_ansi(data, "birthday".to_string()),txt);
    c
}

pub fn colour_event(txt: &str) -> String {
    let data = parse_colour_settings(&settings_file_path());
    let c = format!("{}{}\x1B[0m",convert_to_ansi(data, "event".to_string()),txt);
    c
}

pub fn colour_holiday(txt: &str) -> String {
    let data = parse_colour_settings(&settings_file_path());
    let c = format!("{}{}\x1B[0m",convert_to_ansi(data, "holiday".to_string()),txt);
    c
}

pub fn colour_today(txt: &str) -> String {
    let data = parse_colour_settings(&settings_file_path());
    let c = format!("{}{}\x1B[0m",convert_to_ansi(data, "today".to_string()),txt);
    c
}

pub fn colour_vacations(txt: &str) -> String {
    let data = parse_colour_settings(&settings_file_path());
    let c = format!("{}{}\x1B[0m",convert_to_ansi(data, "vacation".to_string()),txt);
    c
}

pub fn colour_tomorrow(txt: &str) -> String {
    let data = parse_colour_settings(&settings_file_path());
    let c = format!("{}{}\x1B[0m",convert_to_ansi(data, "tomorrow".to_string()),txt);
    c
}


/// colours the text based on the users settings and the given month
/// 
/// ### Input
/// **n**: usize --> the month number (valid numbers are in [0,11])
/// 
/// **txt**: &String --> the text being coloured
/// 
/// ### Return
///  String -->  the coloured text
pub fn colour_month_title(n: usize, txt: &String) -> String {
    let data = parse_colour_settings(&settings_file_path());
    let c = match n {
         0 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "january".to_string()),txt),
         1 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "february".to_string()),txt),
         2 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "march".to_string()),txt),
         3 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "april".to_string()),txt),
         4 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "may".to_string()),txt),
         5 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "june".to_string()),txt),
         6 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "july".to_string()),txt),
         7 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "august".to_string()),txt),
         8 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "september".to_string()),txt),
         9 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "october".to_string()),txt),
        10 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "november".to_string()),txt),
        11 =>format!("{}{}\x1B[0m",convert_to_ansi(data, "december".to_string()),txt),
         _ => String::from(txt) 
    };
    c
}


fn convert_to_ansi(data: HashMap<String, (u8, u8, u8, bool)>, element: String) -> String {
    let element_data = data.get(&element).unwrap();
    let bold = if element_data.3 { "1;" } else { "" };
    
    let r = element_data.0;
    let g = element_data.1;
    let b = element_data.2;
    format!("\x1B[{}38;2;{};{};{}m", bold, r, g, b)
}