use std::cmp::max;

use format::text_fitting::*;

use crate::format;

pub enum ALIGNMENT {
    LEFT,
    RIGHT,
    CENTER
}

// TODO (EVENTUALLY) : 
// make this a struct, and have defaults (left align), etc etc


pub fn text_box(width: usize, height: usize, text: String, alignment: ALIGNMENT) -> Vec<String> {
    let formatted_text = fit_text_to_char_width(&text, width-2);
    vec_box(width, height, &formatted_text, alignment)
}


// NOTE FOR FUTURE : 
// Left align   :  (width - 2 - sentence.len() ) /2
// Right align  :  width - 2 - left_box_space.len() - sentence.len()
// Center align :  both at the same time


pub fn print_box_content_2_cols(left_col: Vec<String>, right_col: Vec<String>) {

    for i in 0..(max(left_col.len(), right_col.len())) {

        let left_content = match left_col.get(i) {
            Some(txt) => txt,
            None => "",
        };

        let right_content = match right_col.get(i) {
            Some(txt) => txt,
            None => "",
        };
    
        println!("{}{}", left_content,right_content);
    
    }

}


/// NOTE: left align = -1, centre = 0, right = 1
pub fn vec_box(width: usize, height: usize, text: &Vec<String>, alignment: ALIGNMENT) -> Vec<String> {

    let (text_fits, problem_str_size) = validate_vector_size(width, &text);
    if !text_fits { panic!("Some text doesn't fit in the box! (max text width = {}, text width = {})", width-2, problem_str_size)}
    if text.len() > height { panic!("Text can't fit in box! (height = {}, number of lines of text = {})", height-2, text.len())}
    
    
    let mut box_content: Vec<String> = vec![];

    let horizontal_edge = "─".repeat(width - 2);
    

    box_content.push(format!("╭{}╮", horizontal_edge));
    
    for i in 2..=height {
        let sentence = match &text.get(i-2) {
            Some(txt) => txt,
            None => ""
        };
    
        let (left_space_size, right_space_size) = match alignment {
            ALIGNMENT::CENTER => center_align_buffers(width as u8, count_visible_chars(sentence) as u8),
            ALIGNMENT::LEFT => left_align_buffers(width as u8, count_visible_chars(sentence) as u8),
            ALIGNMENT::RIGHT => right_align_buffers(width as u8, count_visible_chars(sentence) as u8)
        };

        let left_space = " ".repeat(left_space_size as usize);
        let right_space = " ".repeat(right_space_size as usize);

        box_content.push(format!("│{}{}{}│",left_space, sentence, right_space));
    }
    box_content.push(format!("╰{}╯",horizontal_edge));

    box_content
}




fn left_align_buffers(width: u8, text_width: u8 ) -> (u8, u8) { 

    let left_buffer_size = 1;
    // width - 2 (walls of the box chars) - 1 (size of the left 'buffer') = width - 3
    let right_buffer_size = (width - 3 - text_width) as u8;

    (left_buffer_size, right_buffer_size)
}


fn center_align_buffers(width: u8, text_width: u8) -> (u8, u8) { 

    let blank_area = (width - 2 - text_width) as f32 / 2.0;
    ( blank_area.floor() as u8, blank_area.ceil() as u8 )
}


fn right_align_buffers(width: u8, text_width: u8) -> (u8, u8) { 

    let right_buffer_size = 1;
    let left_buffer_size = (width - 3 - text_width) as u8;

    (left_buffer_size, right_buffer_size)
}