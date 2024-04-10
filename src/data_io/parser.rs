use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};




/// scans the settings file and extracts the colour information on events and month headers
pub fn parse_colour_settings(file_path: &str) -> HashMap<String, (u8, u8, u8, bool)> {
    let mut colours_map = HashMap::new();
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        let mut in_colours_section = false;

        for line in reader.lines() {
            if let Ok(line) = line {
                // Ignore anything after ">>"
                let line = line.split(">>").next().unwrap_or("").trim();
                
                if line.is_empty() { continue; }

                if line == "[colours]" {
                    in_colours_section = true;
                    continue;
                } else if line == "[end colours]" {
                    break;
                }

                if !in_colours_section { continue; }
                
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let colour_name = parts[0];
                    let mut values = parts[1].split(',').map(|s| s.trim());
                    if let (Some(red), Some(green), Some(blue)) = (
                        values.next().and_then(|s| s.parse().ok()),
                        values.next().and_then(|s| s.parse().ok()),
                        values.next().and_then(|s| s.parse().ok()),
                    ) {
                        let bold = values.next().map_or(false, |s| s == "+bold");
                        colours_map.insert(
                            colour_name.to_string(),
                            (red, green, blue, bold),
                        );
                    }
                }
            
            }
        }
    } else {
        println!("Failed to open the file.");
    }
    colours_map
}
