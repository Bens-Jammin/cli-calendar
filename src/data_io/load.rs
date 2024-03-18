use std::fs::File;
use std::io::Read;

fn load_year(file_path: &str) -> std::io::Result<Year> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let year: Year = serde_json::from_str(&contents)?;
    Ok(year)
}
