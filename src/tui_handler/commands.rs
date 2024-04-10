use chrono::{Datelike, Utc};
use crate::{data_io::{load, paths::json_path, save::save_year, std_rw}, display::{self, calendar::year::calendar_year}, structures::{event::Event, year::Year}, tui_handler::terminal_format::clear};


fn year() -> Year { load::load_year(&json_path()).unwrap() }



pub fn help() {

}


pub fn view(options: &[String]) {

    let year = load::load_year(&json_path()).unwrap();
    let calendar = calendar_year(&year);
    match options.get(0).unwrap().as_str() {
        "year" => {
            display::calendar::year::show_year(&calendar);
        }
        "month" => {
            let month = Utc::now().month();
            display::calendar::month::show_3month_window(&year, (month - 1) as usize )
        }
        "week" => todo!("Not implemented yet."),
        _ => display::calendar::year::show_year(&calendar),
    }
}


pub fn add() {
    println!("Enter day number: ");
    let d = (std_rw::stdin_int() - 1) as u8;
    clear(2);
    println!("Enter month number");
    let m = (std_rw::stdin_int() - 1) as u8;
    clear(2);

    let mut year = year();

    println!("Enter event name:");
    let event_name = std_rw::std_str();
    clear(2);
    println!("Enter event description:"); 
    let event_desc = std_rw::std_str();
    clear(2);
    println!("Enter event start time (hour, minute on separate lines and in 24 hour time)");
    let event_start  = std_rw::std_time();
    clear(3);
    // TODO, have std::time take a description as param?
    println!("Enter event end time (hour, minute on separate lines and in 24 hour time)");
    let event_end  = std_rw::std_time();
    clear(3);

    println!("Enter an icon to be displayed in the calendar.\nLeave blank if you do not want an icon.");
    let event_icon = std_rw::std_str();
    clear(3);


    let event = Event { 
        name: String::from(event_name.trim()), 
        description: String::from(event_desc.trim()), 
        start: event_start.clone(), 
        end: event_end.clone(),
        icon: if event_icon.len() == 0 { String::new() } else { String::from(event_icon) } 
    };

    year.add_event(d.clone(),m.clone(),&event);


    match save_year(&year, &json_path()) {
        Ok(_) =>println!("Event successfully saved.\n name: {} \n description: {} \n date: {}-{}, duration: ({}-->{})",event_name, event_desc,d,m, event_start.format(), event_end.format()),
        Err(e) => println!("An error occurred while saving event details -> {}", e.to_string() ),
    }
    year.month(m as usize).day(d as usize).print_events();
    
}


pub fn remove(options: &[String]) {

}


pub fn export(options: &[String]) {

}