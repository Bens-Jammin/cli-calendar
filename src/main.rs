use display::{calendar::year::calendar_year, content_box::{self, ALIGNMENT}, incoming_events::{self, *}, progress_bar};
use format::text_fitting::*;
use structures::{event::Event, holiday::ontario_public_holidays, time24h::Time24h, year::Year};
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
        icon: String::from("𖤐 "),
        name: String::from("New Music"),
        description: String::from("Kerry King new album: 'From Hell I rise'"),
        start: Time24h::default(),
        end: Time24h::new(23,59),
        priority: false
    };

    let event2 = Event{
        icon: String::from(""),
        name: String::from("Date"),
        description: String::from("Test event!!!!! (not a priority)"),
        start: Time24h::default(),
        end: Time24h::new(23, 59),
        priority: false
    };

    let event3 = Event{
        icon: String::from(""),
        name: String::from("Apt. Viewing"),
        description: String::from("House viewing at 260 Mona Ave."),
        start: Time24h::new(14, 0),
        end: Time24h::new(15, 0),
        priority: true
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
 

    let upcoming_events = incoming_events::get_incoming_events(year.current_month(), year.next_month());

    let small_top_box = content_box::vec_box(40, 23,&upcoming_events , ALIGNMENT::LEFT);
    let small_mid_box = content_box::vec_box(40,13, &progress_info, ALIGNMENT::CENTER);
    //let small_bottom_box = content_box::text_box(40,5,0,0, "This box is on the bottom! :3".to_owned());

    let mut right_hand_column: Vec<String> = Vec::new();
    right_hand_column.extend(small_top_box);
    right_hand_column.extend(small_mid_box);
    //right_hand_column.extend(small_bottom_box);

    content_box::print_box_content_2_cols(big_box, right_hand_column);

    // for e in ontario_public_holidays() {
    //     println!("emoji: '{}', width: {}", e.icon(), count_visible_chars(&e.icon()));
    // }
}

