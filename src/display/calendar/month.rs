use std::cmp::max;

use crate::structures::year::Year;
use crate::{structures::month::Month, display::utils};
use super::misc::{get_weeks_formatted, get_weeks_raw};


pub const MONTH_CAL_WIDTH: usize = 26;


/// returns a title of the month
pub fn title(month: &Month) -> String {
    let width: usize = 26;
   let title = String::from(" ")+&month.name + " " + &month.year.to_string() + " ";
   let left_length = (width - title.len()) / 2;
   let right_length = width - title.len() - left_length;

   let result = "-".repeat(left_length) + &title + &"-".repeat(right_length);
   result
}



/// displays previous, current, and last month
pub fn show_3month_window(year: &Year, current_month: usize) {
    println!("\n");
    
    let this_month_vec = month_as_vec(year.month(current_month));
    
    let last_month_vec = if current_month > 0 {
        month_as_vec(year.month(current_month - 1)) } else { vec![] };
    let next_month_vec = if current_month < 11 {
        month_as_vec(year.month(current_month + 1)) } else { vec![] };

    let legend = utils::highlight_legend();

    let num_rows_to_print = max(last_month_vec.len(), max(this_month_vec.len(), next_month_vec.len()) );

    for i in 0..num_rows_to_print {
        let empty_week = String::from("  ") + &" ".repeat(MONTH_CAL_WIDTH);
        let l = last_month_vec.get(i).unwrap_or(&empty_week);
        let t = this_month_vec.get(i).unwrap_or(&empty_week);
        let n = next_month_vec.get(i).unwrap_or(&empty_week);

        let h_l = legend.get(i);
        let h = match h_l {
            Some(value) => value,
            None => "", // Empty string if h_l is None
        };

        println!("{} {} {}   {}", l, t, n, h);
    }
}

pub fn month_as_vec(m: &Month) -> Vec<String> {
    let mut month: Vec<String> = vec![
        title(m),
        String::from("MO  TU  WE  TH  FR  SA  SU"),
    ]; 
    let mut formatted_weeks = get_weeks_formatted(m).into_iter();
    for week in get_weeks_raw(m) {
        let week_vec: Vec<char> = week.chars().collect();

        let padding_width: isize = MONTH_CAL_WIDTH as isize - (week_vec.len() as isize);
        
        let formatted_week = formatted_weeks.next().unwrap();
        
        let padded_week = if padding_width < 0 {  
            String::from(formatted_week) 
        } else {
            String::from(formatted_week) + "  " + &" ".repeat(padding_width as usize) 
        };
        month.push(padded_week);
    }

    month
}