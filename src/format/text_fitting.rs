

pub fn fit_text_to_char_width(txt: &String, width: usize) -> Vec<String> {

    let words: Vec<&str> = txt.split(" ").collect();

    let mut formatted_text: Vec<String> = vec![];

    let mut current_line = String::new();

    for word in words {
        let start_new_line = word.ends_with('\n');
        
        let visible_char_count = count_visible_chars(word);
        
        if word.starts_with('\n') {
            formatted_text.push(current_line);
            current_line = String::from("");
        }
        let word  = word.replace('\n', "");
        
        if count_visible_chars(&current_line) + visible_char_count < width {
            current_line += &format!("{} ", word);
            
        } else {
            formatted_text.push(current_line);
            current_line = format!("{} ", word);
        }
        
        if start_new_line {
            formatted_text.push(current_line);
            current_line = String::from("");
        }
    }
    if !(current_line.len() == 0 || current_line == "") {  formatted_text.push(current_line); }
    return formatted_text;
}



pub fn validate_vector_size(max_string_size: usize, vec: &Vec<String> ) -> (bool, usize) {

    for str in vec.iter() {
        let visible_char_count = count_visible_chars(&str);
        if visible_char_count > max_string_size { return (false, visible_char_count); }
    }
    (true, 0)
}


pub fn count_visible_chars(s: &str) -> usize {
    let mut count = 0;
    let mut in_escape = false;
    let mut long_escape = false;

    for c in s.chars() {
        if in_escape && !long_escape && c == '[' {
            long_escape = true;
        }
        if c == '\x1b' {
            in_escape = true;
        }
        if !in_escape && !long_escape {
            count += 1;
        }
        if in_escape && !long_escape && c != '\x1b' {
            in_escape = false;
        }
        if long_escape && c != '[' && c >= '@' && c <= '~' {
            in_escape = false;
            long_escape = false;
        }
    }

    count
}
