pub fn handle_key (key: char, longkey: &str) {
    let mut wasdmode = true;
    if wasdmode {
        match key {
            'w' => {},
            'a' => {},
            's' => {},
            'd' => {},
            _ => {},
        };
        // https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals
        match longkey {
            "Left" => {},
            "Right" => {},
            "Up" => {},
            "Down" => {},
            _ => { },
        };
    } else {
        match key {
            'w' => {},
            'a' => {},
            's' => {},
            'd' => {},
            _ => {},
        };
        match longkey {
            "Left" => {},
            "Right" => {},
            "Up" => {},
            "Down" => {},
            _ => { },
        };
    }
    match longkey {
        "Space" => wasdmode = !wasdmode,
        "Tab" => wasdmode = !wasdmode,
        _ => { },
    }
    println!("{}, and {}, oh and also {}", key, longkey, wasdmode)
}