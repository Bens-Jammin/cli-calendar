use crate::dates::structs::*;


impl Month {

    pub fn new(month_no: usize, year_no: usize) -> Month {
        Month {
            number: month_no,
            year: year_no, 
            name: Month::name_from_month_number(month_no).to_owned(),
            days: Month::make_days(month_no, year_no)
        }
    }

    pub fn empty(month_no: usize, year_no: usize) -> Month {
        Month {
            number: month_no,
            year: year_no, 
            name: Month::name_from_month_number(month_no).to_owned(),
            days: vec![]
        }
    }

    pub fn num_days(month_no: usize, year: usize) -> usize {
        match month_no {
            1 => 31,
            2 => {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } 
                else { 28 }
            }
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => unreachable!(),
        }
    }

    fn make_days(month_no: usize, year: usize) -> Vec<Day> {
        let mut vec = vec![];


        for d in 1..Month::num_days(month_no, year) {
            vec.push(
                Day::new(d, month_no, year, Holiday::None)
            );
        }
        vec
    }
   
    pub fn name_from_month_number( month_no: usize) -> String {
        match month_no {
            1 => String::from("January"),
            2 => String::from("February"),
            3 => String::from("March"),
            4 => String::from("April"),
            5 => String::from("May"),
            6 => String::from("June"),
            7 => String::from("July"),
            8 => String::from("August"),
            9 => String::from("September"),
            10 => String::from("October"),
            11 => String::from("November"),
            12 => String::from("December"),
            _ => panic!("Invalid month number"),       
        }
    }
}