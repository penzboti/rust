use crossterm::cursor::MoveUp;
use crossterm::terminal::size;
use std::io::stdout;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;
use text_to_ascii_art::to_art;

#[macro_use]
extern crate crossterm;

fn main() {
    let mut speed = 10;
    let mut text: String = "Text here".to_owned();

    // args
    let args: Vec<String> = std::env::args().collect();

    let mut args_offset = 1;
    if args.len() > 2 && (args[1] == "-s" || args[1] == "--speed") {
        speed = args[2].parse::<u64>().expect("Couldn't parse the speed.");
        args_offset += 2;
    }
    if args[1] == "-h" || args[1] == "--help" {
        println!("Usage: write [options] <text>");
        println!("\r\nOptions:");
        println!(
            "-speed, -s <speed>      Set the millisecond difference between animation frames."
        );
        return;
    }
    if args.len() > args_offset {
        let mut str: String = "".to_string();
        for (i, arg) in args.iter().enumerate() {
            if i < args_offset {
                continue;
            }
            str.push_str(arg);
            str.push_str(" ");
        }
        str.pop();
        text = str;
    }

    // get terminal size
    let (width, _) = size().expect("Can't get terminal size. Exiting...");

    let ascii = to_art(text, "", width as usize, 0, 0)
        .expect("Can't turn your text into ascii. Exiting...");
    let mut v: Vec<&str> = ascii.split('\n').collect();
    // remove empty line at the start
    v.reverse();
    v.pop();
    v.reverse();

    let line_width = v[0].chars().count(); // hey, did you know that .len() doesn't return the char length? it returns byte length.
    let max_index: usize = line_width + 1;
    let mut index: usize = 0; // current offset from the start

    // handling ctrl+c
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!();
    while running.load(Ordering::SeqCst) {
        if index == max_index {
            break;
        }
        for line in v.clone() {
            for i in 0..width {
                print!("{}", line.chars().nth(i as usize + index).unwrap_or(' '));
            }
            println!();
        }
        index += 1;
        sleep(Duration::from_millis(speed));
        execute!(stdout(), MoveUp(v.len() as u16)).expect("Failed to restore cursor position");
    }
    println!("exited :)")
}
