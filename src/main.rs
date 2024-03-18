mod display;
mod dates;


use display::utils;

use crate::{dates::structs::Month, display::calendar, dates::holidays};


fn main() {

    let m = Month::new(3, 2024);


    utils::display_calendar_with_highlighting(&m);

}
