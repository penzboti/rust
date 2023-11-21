// disabling warning messages
// https://www.reddit.com/r/rust/comments/cfvr7p/i_have_a_lot_of_unused_import_warning_how_do_i/?rdt=33823
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![warn(unused_assignments)]
#![warn(unused_mut)]

// imports, these make the program overall shorter
use std::io::{Write, stdout};
// crossterm
// https://crates.io/crates/crossterm 
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, };
use crossterm::event::{self, Event};

// for importing external macros (you can also import them normally btw)
#[macro_use]
extern crate crossterm;

// useful for debugging, and also couldn't rewrite, so this stays as a function
// https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
pub fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}


// importing modules
// https://youtu.be/969j0qnJGi8?si=xfPuPJfgoRw_Woud
mod keyboard;
mod render;
pub mod handle_move;
mod cmd;

// this exists raw_mode if the program crashes, or probably on exit aswell
// https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237
struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() -> std::io::Result<()> {
    
    //TODO: public variables
    let mut pointer_xpos: u16 = 0;
    let mut pointer_ypos: u16 = 0;
    let mut target_xpos: u16 = 0;
    let mut target_ypos: u16 = 0;
    let mut wasdmode = true;
    
    let _clean_up = CleanUp;
    let mut stdout = stdout();
    enable_raw_mode().expect("Could not turn on Raw mode");
    // massive help on keyboard events from this guide
    // https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237
    // and also some help from this guide
    // https://stackoverflow.com/a/60130920/12706133
    // but they were both outdated
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match keyboard::match_keyboard_event(event) {
                None => {},
                Some(val) => if val {break;},
            }
            // for debugging purposes
            // println!("{:?}\r", event);
        }
    }

    // exiting program
    stdout.flush()?;
    Ok(())
}