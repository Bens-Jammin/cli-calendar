

pub fn is_valid_emoji(s: &String) -> bool {
    valid_emojis().contains(s) || s.is_empty()
}


fn valid_emojis() -> Vec<String> {
    vec![
        String::from("🎄"),
        String::from("⚒️"),
        String::from("🦃"),
        String::from("🎃"),
        String::from("🍁"),
        String::from("✝️"),
        String::from("❤️"),
        String::from("🎉")
    ]
}