use data_io::{load::load_year, paths::json_path};
use display::{calendar::year::{calendar_year, show_year}, content_box::{self, ALIGNMENT}, progress_bar};
use structures::{event::Event, time24h::Time24h, year::Year};
mod tui_handler;
mod display;
mod structures;
mod data_io;
mod format;


fn main() {

    // let mut year = load_year(&json_path()).unwrap();
    // year.clear_all();
    // let _ = save_year(&year, &json_path());

    let mut year = Year::new(); 
    let event = Event{
        icon: String::from("𖤐"),
        name: String::from("New Music"),
        description: String::from("Kerry King new album: 'From Hell I rise'"),
        start: Time24h::default(),
        end: Time24h::new(23,59)
    };

    let event2 = Event{
        icon: String::from(""),
        name: String::from("Date"),
        description: String::from("Watching 'Saltburn' with Remy"),
        start: Time24h::default(),
        end: Time24h::new(23, 59)
    };

    let event3 = Event{
        icon: String::from(""),
        name: String::from("Apt. Viewing"),
        description: String::from("House viewing at 260 Mona Ave."),
        start: Time24h::new(14, 0),
        end: Time24h::new(15, 0)
    };
    
    year.add_event(17,4, &event2);
    year.add_event(15,5, &event);
    year.add_event(13, 4, &event3);

    
    let cal = calendar_year(&year);
    let big_box = content_box::vec_box(120, 37, &cal, ALIGNMENT::CENTER);

    let progress_info = vec![
        String::from("Current Year progress:"),
        progress_bar::year_progress(false),
        String::from(""),
        String::from("Current month progress:"),
        progress_bar::month_progress(false),
        String::from(""),
        String::from("Current day progress:"),
        progress_bar::day_progress(false),
    ];
 

    let small_top_box = content_box::text_box(40, 23, "test".to_owned(), ALIGNMENT::LEFT);
    let small_mid_box = content_box::vec_box(40,13, &progress_info, ALIGNMENT::CENTER);
    //let small_bottom_box = content_box::text_box(40,5,0,0, "This box is on the bottom! :3".to_owned());

    let mut right_hand_column: Vec<String> = Vec::new();
    right_hand_column.extend(small_top_box);
    right_hand_column.extend(small_mid_box);
    //right_hand_column.extend(small_bottom_box);

    content_box::print_box_content_2_cols(big_box, right_hand_column);


    // let args = vec!["program_name".to_string(), "add".to_string()];
    // handle_initial_args(args);

}

