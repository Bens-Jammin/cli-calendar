use chrono::Datelike;
use crate::dates::structs::Month;

pub fn show_upcoming_events(month: &Month) {
    let today = chrono::Utc::now().day();
    let m = Month::name_from_month_number(chrono::Utc::now().month().try_into().unwrap());
    let y = chrono::Utc::now().year();

    let date = match today % 7 {
        0 => "Monday, ",
        1 =>  "Tuesday, ",
        2 =>  "Wednesday, ",
        3 =>  "Thursday, ",
        4 =>  "Friday, ",
        5 =>  "Saturday, ",
        6 =>  "Sunday, ",
        _ =>  panic!("Error parsing date.")
    };

    let formatted_date = format!("{}{} {}, {}", date, m, today, y);
    println!("\n{}\n\nUpcoming events:",formatted_date);

    for d in &month.days {
        if (today..=today + 7).contains(&(d.num as u32)) {
            d.print_events();
        }
    }
}
