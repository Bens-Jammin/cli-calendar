use chrono::Datelike;

/// grabs the settings file for the given user
/// 
/// ### NOTE
/// As of writing this (March 25, 2024), this function supports one user (me)
/// 
/// for anyone in the future, the file must be in the users file directory (named after their account), in the directory 'users'.
/// 
/// the file must be called 'settings' and have the '.bmsf' file extension (Ben Miller Style Format)
/// 
/// 
/// ### Valid File Example
/// 
/// 'users/ben/settings.bmsf'
/// 
/// 
/// ### Invalid File Example
/// 
/// 'users/settings.bmsf'       --> no account sub directory
/// 
/// 'ben/settings.bmsf'         --> not in the 'users' directory
/// 
/// 'users/ben/settings.txt'    --> invalid file format
/// 
/// 'users/ben/styles.bmsf'     --> invalid file name
pub fn settings_file_path() -> String {
    let account = "ben";
    format!("users/{}/settings.bmsf",account)   // ben miller calendar settings
}


pub fn json_path() -> String {
    let account = "ben";
    let year = chrono::Utc::now().year();
    format!("users/{}/{}-{}.json", account, account, year)
}