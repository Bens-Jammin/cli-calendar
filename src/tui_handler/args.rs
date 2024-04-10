use crate::tui_handler::commands;




pub fn handle_initial_args(args: Vec<String>) {
    if args.len() < 1 {
        println!("Wrong Argument structure.");
        commands::help();
        return;
    }

    let arg_options = &args[2..];
    match args.get(1).unwrap().as_str() {
        "help"   => commands::help(),
        "see"    => commands::view(arg_options),
        "add"    => commands::add(),
        "remove" => commands::remove(arg_options),
        "export" => commands::export(arg_options),
        _        => commands::help(),
    }
}
