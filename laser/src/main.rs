// disabling warning messages
// https://www.reddit.com/r/rust/comments/cfvr7p/i_have_a_lot_of_unused_import_warning_how_do_i/?rdt=33823
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![warn(unused_assignments)]
#![warn(unused_mut)]

// imports, these make the program overall shorter
use std::io::{Write, stdout};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
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
        disable_raw_mode().expect("Could not disable raw mode");
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

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, crossterm::cursor::Hide)?;
    enable_raw_mode().expect("Could not turn on Raw mode");

    let mut pointer: Pos = Pos {x: 1, y: 1};
    let mut target: Pos = Pos {x: 2, y: 2};
    let mut wasdmode: bool = true;

    render::render(&pointer, &target);
    loop {
        // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
        // this also cleares main screen
        // print!("{esc}c", esc = 27 as char);
        // this goes to 0,0 cursor pos
        // print!("\x1B[2J\x1B[1;1H");
        // should be the same? but isnt but whatever
        // execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
        // crossterm way
        execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge) )?;
        // wanted to do it this way but realized it doesnt work how i wanted it to
        // https://unix.stackexchange.com/questions/360198/can-i-overwrite-multiple-lines-of-stdout-at-the-command-line-without-losing-term

        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match keyboard::match_keyboard_event(event) {
                Some(val) => {
                    let key = val.as_str();
                    if key != "" {
                        if key == "Space" {
                            wasdmode = !wasdmode;
                            continue;
                        }
                        if key == "Tab" {
                            wasdmode = !wasdmode;
                            continue;
                        }
                        let diff = position::handle_key(String::from(key), wasdmode);
                        match diff.0.as_str() {
                            "pointer" => {
                                let ans = position::update_val((diff.1, diff.2), (pointer.x, pointer.y));
                                match ans {
                                    Some(val) => {
                                        pointer.x = val.0;
                                        pointer.y = val.1;
                                    },
                                    None => {},
                                }
                            },
                            "target" => {
                                let ans = position::update_val((diff.1, diff.2), (target.x, target.y));
                                match ans {
                                    Some(val) => {
                                        target.x = val.0;
                                        target.y = val.1;
                                    },
                                    None => {},
                                }
                            },
                            _ => {},
                        }
                        render::render(&pointer, &target);
                    }
                },
                None => break,
            }
            // for debugging purposes, printing every event
            // println!("{:?}\r", event);
        }

    }

    // exiting program
    execute!(stdout, LeaveAlternateScreen)?;
    execute!(stdout, crossterm::cursor::Show)?;
    stdout.flush()?;
    Ok(())
}