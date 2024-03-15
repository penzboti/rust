pub fn handle_key (key_string: String, wasdmode: bool) -> (String, i16, i16) {
    let mut diff: (&str, i16, i16) = ("", 0, 0);
    let key = key_string.as_str();

    if wasdmode {
        match key {
            "w_key" => { diff = ("pointer", 0, -1); },
            "a_key" => { diff = ("pointer", -1, 0); },
            "s_key" => { diff = ("pointer", 0, 1); },
            "d_key" => { diff = ("pointer", 1, 0); },
            _ => { },
         };
        // https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals
        match key {
            "Left" => { diff = ("target", -1, 0); },
            "Right" => { diff = ("target", 1, 0); },
            "Up" => { diff = ("target", 0, -1); },
            "Down" => { diff = ("target", 0, 1); },
            _ => {  },
         };
     } else {
        match key {
            "w_key" => { diff = ("target", 0, -1); },
            "a_key" => { diff = ("target", -1, 0); },
            "s_key" => { diff = ("target", 0, 1); },
            "d_key" => { diff = ("target", 1, 0); },
            _ => { },
         };
        match key {
            "Left" => { diff = ("pointer", -1, 0); },
            "Right" => { diff = ("pointer", 1, 0); },
            "Up" => { diff = ("pointer", 0, -1); },
            "Down" => { diff = ("pointer", 0, 1); },
            _ => { },
        };
    }

    (diff.0.to_string(), diff.1, diff.2)

}

pub fn update_val( diff: (i16, i16), val: (u16, u16) ) -> Option<(u16, u16)> {
    let size = crate::render::get_terminal_size();
    let iresult = (val.0 as i16 + diff.0, val.1 as i16 + diff.1);

    if iresult.0 >= 1 && iresult.1 >= 0 {
        let uresult = (iresult.0 as u16, iresult.1 as u16);
        if uresult.0 <= size["cols"]-1 && uresult.1 <= size["rows"]-1 {
            return Some(uresult);
        }
    }
    
    None
}