use std::sync::atomic::Ordering;
use crate::publicvars::WASDMODE;
use crate::publicvars::{P_XPOS, P_YPOS, T_XPOS, T_YPOS, update_var};

pub fn handle_key (key: char, longkey: &str) {
    let mut wasdmode = WASDMODE.load(Ordering::Relaxed);
    let mut res: bool = true;
    if wasdmode {
        match key {
            'w' => { res = update_var(&P_YPOS, -1, 'y'); },
            'a' => { res = update_var(&P_XPOS, -1, 'x'); },
            's' => { res = update_var(&P_YPOS, 1, 'y'); },
            'd' => { res = update_var(&P_XPOS, 1, 'x'); },
            _ => { },
         };
        // https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals
        match longkey {
            "Left" => { res = update_var(&T_XPOS, -1, 'x'); },
            "Right" => { res = update_var(&T_XPOS, 1, 'x'); },
            "Up" => { res = update_var(&T_YPOS, -1, 'y'); },
            "Down" => { res = update_var(&T_YPOS, 1, 'y'); },
            _ => {  },
         };
     } else {
        match key {
            'w' => { res = update_var(&T_YPOS, -1, 'y'); },
            'a' => { res = update_var(&T_XPOS, -1, 'x'); },
            's' => { res = update_var(&T_YPOS, 1, 'y'); },
            'd' => { res = update_var(&T_XPOS, 1, 'x'); },
            _ => { },
         };
        match longkey {
            "Left" => { res = update_var(&P_XPOS, -1, 'x'); },
            "Right" => { res = update_var(&P_XPOS, 1, 'x'); },
            "Up" => { res = update_var(&P_YPOS, -1, 'y'); },
            "Down" => { res = update_var(&P_YPOS, 1, 'y'); },
            _ => { },
        };
    }
    match longkey {
        "Space" => wasdmode = !wasdmode,
        "Tab" => wasdmode = !wasdmode,
        _ => { },
    }

    WASDMODE.store(wasdmode, Ordering::Relaxed);

    match res {
        true => crate::render::render(),
        false => {},
    }
}