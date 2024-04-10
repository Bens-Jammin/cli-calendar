use std::fs::File;
use std::io::Write;

use crate::structures::year::Year;

pub fn save_year(year: &Year, file_path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string(&year)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

