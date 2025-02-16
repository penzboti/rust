const SLEEP_MS: u64 = 10;
const PRINT_I: bool = false;
const HEIGHT: u32 = 31;

// options setup
struct Options {
    speed: u64,
    numbers: bool,
    width: u32,
    length: u32,
}
impl Options {
    fn new(size: u32) -> Options {
        Options {
            speed: SLEEP_MS,
            numbers: PRINT_I,
            width: size / 8,
            length: HEIGHT,
        }
    }
}

// toggling cursor
// https://stackoverflow.com/questions/55578765/is-there-possible-way-to-hide-cursor-inter-run-time-in-console-in-fortran
struct Cursor;
impl Drop for Cursor {
    fn drop(&mut self) {
        println!("exiting...");
        // re-enables cursor
        print!("\x1B[?25h");
        // resets terminal color
        // print!("\x1B[0m");
    }
}
impl Cursor {
    fn new() -> Cursor {
        // TODO: color
        // set color: requires TRUECOLOR terminal
        // bg is 48;
        // print!("\x1b[38;2;255;255;0m");

        // disable cursor
        print!("\x1B[?25l");
        return Cursor;
    }
}

use std::f32::consts::PI;
// for ctrl-c
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn getsine(x: f32) -> f32 {
    return f32::sin(x);
}

fn main() {
    // terminal size stuff
    let size = termsize::get().unwrap();
    let centerx = size.cols / 2 - 1;

    let mut options = Options::new(size.cols as u32);
    // command line arguments
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    loop {
        if args.len() == 0 {
            break;
        }
        let arg = args[0].clone();
        args.remove(0);
        if arg == "-n" || arg == "--numbers" {
            options.numbers = true;
            continue;
        }
        if arg == "-h" || arg == "--help" {
            let msg = "Sine wave
-h --help  ->  display this help message
-n --numbers  ->  display the current iteration
-w --width <num>  ->  set the amount of characters in one direction. The default is 25% of the screen width.
-l --length <num>  ->  set the iteration length for one arc.";
            for line in msg.split('\n') {
                println!("{}", line);
            }
            return;
        }

        let val = args[0].clone().parse().unwrap();
        args.remove(0);
        if arg == "-w" || arg == "--width" {
            options.width = val;
        }
        if arg == "-l" || arg == "--length" {
            options.length = val;
        }
        if arg == "-s" || arg == "--speed" {
            options.speed = val as u64;
        }
    }

    // ctrl-c handling
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // initializing the toggling cursor
    let _cursor = Cursor::new();

    // main loop
    let mut i: u16 = 0;
    while running.load(Ordering::SeqCst) {
        let x = getsine(i as f32 / (options.length as f32 / PI)) * options.width as f32;
        let offset_before = x.round() as usize;
        let offset_after = -x.round() as usize;
        // print a pretty $i
        let si = if options.numbers {
            print!("{}", i);
            i.to_string().len()
        } else {
            0
        };

        for _ in 0..centerx as usize - offset_after - si {
            print!(" ");
        }
        for _ in 0..offset_before {
            print!("∙");
        }
        //
        print!("X");
        //
        for _ in 0..offset_after {
            print!("∙");
        }
        println!();

        std::thread::sleep(std::time::Duration::from_millis(options.speed));
        i += 1;
    }
}
