use std::io;

use crate::structures::time24h::Time24h;



pub fn stdin_int() -> i32 {
    let mut result = String::new();
    
    io::stdin()
    .read_line(&mut result)
    .expect("Failed to read line");
    
    result
    .trim()
    .parse()
    .unwrap()
}


pub fn std_str() -> String {
    let mut result = String::new();

    io::stdin()
    .read_line(&mut result)
    .expect("Failed to read input.");

    let _ = result.trim();
    result
}


pub fn std_time() -> Time24h {
    let h = stdin_int() as usize;
    let m = stdin_int() as usize;

    Time24h::new(h, m)
}


pub fn std_bool() -> bool {
    let ans = std_str();
    ans.to_lowercase() == String::from("Y")
} 