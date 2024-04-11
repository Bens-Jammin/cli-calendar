use std::cmp::max;
use crate::{display::{calendar::month::{month_as_vec, MONTH_CAL_WIDTH}, colours::*, incoming_events, progress_bar, utils}, structures::year::Year};



pub fn calendar_year(year: &Year) -> Vec<String>{
    
    let year_label_width = ( MONTH_CAL_WIDTH * 3 ) + 4; 
    let big_year_label = String::from("\x1b[1m")+&"=".repeat((year_label_width-6)/2) + " "+&year.year.to_string() +" "+&"=".repeat((year_label_width-6)/2)+&String::from("\x1b[0m");
    
    let mut calendar: Vec<String> = vec![
        String::from(""),   // just to make any boxes feel less crowded
        big_year_label
    ];

    let mut can_print_month_title = true;

    for m in (0..10).step_by(4) {
        let m1 = month_as_vec(year.month(m));
        let m2 = month_as_vec(year.month(m + 1));
        let m3 = month_as_vec(year.month(m + 2));
        let m4 = month_as_vec(year.month(m + 3));


        let num_rows_to_print = max(m1.len(), max(m2.len(), max(m3.len(), m4.len())));

        for w in 0..num_rows_to_print {
            let empty_week = " ".repeat(MONTH_CAL_WIDTH);

            let w1 = m1.get(w).unwrap_or(&empty_week);
            let w2 = m2.get(w).unwrap_or(&empty_week);
            let w3 = m3.get(w).unwrap_or(&empty_week);
            let w4 = m4.get(w).unwrap_or(&empty_week);

            if can_print_month_title {
                calendar.push(format!("{}  {}  {}  {}",
                    colour_month_title(m, w1), 
                    colour_month_title(m+1, w2),
                    colour_month_title(m+2, w3),
                    colour_month_title(m+3, w4)
                ));
                can_print_month_title = false;
            } else {
                calendar.push(format!("{}  {}  {}  {}", w1, w2, w3, w4));
            }            
        }


        can_print_month_title = true;
        let padding = " ".repeat(MONTH_CAL_WIDTH);
        calendar.push(format!("{}  {}  {}  {}", padding, padding, padding, padding));
    }    
    calendar
}


pub fn show_year(cal: &Vec<String>) {
    for row in cal {
        println!("{}", row);
    }
}