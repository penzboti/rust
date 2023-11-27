// disabling warning messages
// https://www.reddit.com/r/rust/comments/cfvr7p/i_have_a_lot_of_unused_import_warning_how_do_i/?rdt=33823
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![warn(unused_assignments)]
#![warn(unused_mut)]

use std::collections::HashMap;
// imports, these make the program overall shorter
use std::io::{Write, stdout};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, };
use crossterm::event::{self, Event};

// for importing external macros (you can also import them with normal 'use' btw)
#[macro_use]
extern crate crossterm;

// useful for debugging, and also couldn't rewrite, so this stays as a function
// https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
pub fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// this exists raw_mode if the program crashes, or probably on exit aswell
// https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237
struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not disable raw mode")
    }
}

// importing modules
// https://youtu.be/969j0qnJGi8?si=xfPuPJfgoRw_Woud
mod keyboard;
mod position;
mod render;
mod cmd;

#[derive(Debug)]
pub struct Pos {
    x: u16,
    y: u16,
}
#[derive(Debug)]
struct Keys {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
// https://users.rust-lang.org/t/how-to-iterate-over-fields-of-struct/53356/2
impl Keys {
    fn iter(&self) -> [(String, bool); 8] {
        [
            ("w_key".to_string(), self.w),
            ("a_key".to_string(), self.a),
            ("s_key".to_string(), self.s),
            ("d_key".to_string(), self.d),
            ("Up".to_string(), self.up),
            ("Down".to_string(), self.down),
            ("Left".to_string(), self.left),
            ("Right".to_string(), self.right),
        ]
    }
}

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    let mut stdout = stdout();
    enable_raw_mode().expect("Could not turn on Raw mode");

    let mut pointer: Pos = Pos {x: 1, y: 1};
    let mut target: Pos = Pos {x: 2, y: 2};
    let mut dummy: Pos = Pos {x: 0, y: 0};
    let mut wasdmode: bool = true;
    let mut keys:Keys = Keys {w: false, a: false, s: false, d: false, up: false, down: false, left: false, right: false};

    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match keyboard::match_keyboard_event(event) {
                Some(val) => {
                    let key = val.as_str();
                    if key != "" {
                        match key {
                            "w_key" => { keys.w = !keys.w;  },
                            "a_key" => { keys.a = !keys.a;  },
                            "s_key" => { keys.s = !keys.s;  },
                            "d_key" => { keys.d = !keys.d;  },
                            "Up" => { keys.up = !keys.up;  },
                            "Down" => { keys.down = !keys.down;  },
                            "Left" => { keys.left = !keys.left;  },
                            "Right" => { keys.right = !keys.right;  },
                            "Space" => { wasdmode = !wasdmode; },
                            "Tab" => { wasdmode = !wasdmode; },
                            _ => { },
                        }
                    }
                },
                None => break,
            }
            // for debugging purposes, printing every event
            // println!("{:?}\r", event);
        }

        // getting the key input still block threads, so i'll use tokio.
        // seems like this one guy has kind of the same problem
        // also could use tui for fixing raw_input_mode? i mean let's see what this guy doeas, and the i'll decide
        // https://users.rust-lang.org/t/text-mode-terminal-application-with-asynchronous-input-output/74760
        let mut do_we_render = false;
        for k in keys.iter() {
            if k.1 {
                let diff = crate::position::handle_key(k.0, wasdmode);
                let ptr: &mut Pos;
                let mut val: (u16, u16) = (0, 0);
                //TODO: make them never overlap each other
                match diff.0.as_str() {
                    "target" => { val = (target.x, target.y); ptr = &mut target; },
                    "pointer" => { val = (pointer.x, pointer.y); ptr = &mut pointer; },
                    // we have to do this because of the compiler, there is something weird going on with string literals. but it doesn't know that we're not gonna use this dummy variable
                    // (thank you, copilot)
                    _ => { ptr = &mut dummy; }
                }
                let res = crate::position::update_val((diff.1, diff.2), val);
                match res {
                    Some(new_val) => { ptr.x = new_val.0; ptr.y = new_val.1; do_we_render = true; },
                    None => { },
                }
            }
        }
        if do_we_render { crate::render::render(&pointer, &target); }
    }

    // exiting program
    stdout.flush()?;
    Ok(())
}