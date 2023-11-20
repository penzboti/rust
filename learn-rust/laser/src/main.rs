// disabling warning messages
// https://www.reddit.com/r/rust/comments/cfvr7p/i_have_a_lot_of_unused_import_warning_how_do_i/?rdt=33823
#![allow(unused_imports)]
// #![allow(unused_variables)]
#![allow(dead_code)]

// imports, these make the program overall shorter
use std::io::{Write, stdout, Read, stdin};
use std::process::Command;
use std::collections::HashMap;
// also, some crossterm docs:
// https://crates.io/crates/crossterm 
use crossterm::terminal::{size as termsize, enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

// for importing external macros (you can also import them normally btw)
#[macro_use]
extern crate crossterm;

// useful for debugging, and also couldn't rewrite, so this stays as a function
// https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// get terminal size with crossterm
fn get_terminal_size() -> HashMap<String, u16>{
    // ty for showing me hashmaps
    // https://stackoverflow.com/a/55376723/12706133
    // &
    // cool hashmap tutorial
    // https://www.koderhq.com/tutorial/rust/hashmap/
    let mut size: HashMap<String, u16> = HashMap::new();
    // how to get a value form a result
    // https://stackoverflow.com/questions/63859927/how-to-get-a-value-from-a-result
    match termsize() {
        Ok(size_tup) => {
            size.insert("rows".to_string(), size_tup.1);
            size.insert("cols".to_string(), size_tup.0);
        },
        Err(e) => eprintln!("{:?}", e),
    }
    size
}

//TODO rewrite for future rendering
// fn render_screen_size(cols: u16, rows: u16) {
//     let mut xrow: String = "".to_string();
//     let xcol = "x";
    
//     for _ in 1..(cols + 1) {
//         xrow = xrow + "x";
//     }
//     println!("{}", xrow);
    
//     for _ in 1..rows {
//         println!("{xcol}")
//     }
// }

// basic functions in command lines
fn pause() {
    // 'pause' command on windows
    // https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/2// 
    // maybe also do clearign this way?
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
fn clear() -> std::io::Result<()> {
    // basically adds a couple of rows, so that earlier information gets off the screen lol. 
    // I don't think this is how it's supposed to work, but at this point i'm afraid to ask
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;
    Ok(())
}

// this exists raw_mode if the program crashes, or probably on exit aswell
// https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237
struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() -> std::io::Result<()> {
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
            match event {
                // documentation on keyEvents
                // https://docs.rs/crossterm/0.27.0/crossterm/event/struct.KeyEvent.html
                // controlling one
                KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("w"),
                KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("a"),
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("s"),
                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("d"),
                // controlling the other
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("↑"),
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("←"),
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("↓"),
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("→"),
                // switch inputs
                KeyEvent {
                    code: KeyCode::Tab,
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("Tab"),
                KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => println!("Space"),
                // break out from program
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => break,
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: event::KeyModifiers::NONE,
                    kind: event::KeyEventKind::Press,
                    state: event::KeyEventState::NONE
                } => break,
                // cuz compiler asked for it, idk
                _ => { continue;
                }
            }
            // for debugging purposes
            // println!("{:?}\r", event);
        }
    }

    // exiting program
    stdout.flush()?;
    Ok(())
}