// options setup
struct Options {
    speed: u64,
    numbers: bool,
    color: bool,
    width: u32,
    length: u32,
}
impl Options {
    fn new(size: u32) -> Options {
        let speed_ms: u64 = 10;
        let height: u32 = 31; // this is set to pi*10
        Options {
            speed: speed_ms,
            numbers: false,
            color: false,
            width: size,
            length: height,
        }
    }
}

fn handle_args(mut args: Vec<String>, size: u32) -> Option<Options> {
    let mut options = Options::new(size);
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
            let msg = "Sine wave in the middle of your screen.
-h, --help\t\tDisplay this help message.
-n, --numbers\t\tDisplay the current iteration.
-c, --colo[u]r\t\tEnable rainbow coloring for the text.
-w, --width <num>\tSet the amount of characters in one direction. The default is 1/8th of the screen width.
It will crash if you set it to more than half of your screen length.
-l, --length <num>\tSet the iteration length for one arc. The default is 31 (pi*10).";
            for line in msg.split('\n') {
                println!("{}", line);
            }
            return None;
        }
        if arg == "-c" || arg == "--color" || arg == "--colour" {
            options.color = true;
            continue;
        }

        let stringval = args[0].clone();
        args.remove(0);
        let val = stringval.parse().unwrap_or(1);
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
    return Some(options);
}

// toggling cursor
// https://stackoverflow.com/questions/55578765/is-there-possible-way-to-hide-cursor-inter-run-time-in-console-in-fortran
struct Cursor;
impl Drop for Cursor {
    fn drop(&mut self) {
        // resets terminal color
        print!("\x1B[0m");
        println!("exiting...");
        // re-enables cursor
        print!("\x1B[?25h");
    }
}
impl Cursor {
    fn new() -> Cursor {
        // disable cursor
        print!("\x1B[?25l");
        return Cursor;
    }
}

// for ctrl-c
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use std::f32::consts::PI;
fn getsine(x: f32) -> f32 {
    return f32::sin(x);
}

fn main() {
    // terminal size stuff
    let size = termsize::get().unwrap();
    let centerx = size.cols / 2 - 1;

    // command line arguments
    let args: Vec<String> = std::env::args().collect();
    let options = handle_args(args, size.cols as u32 / 8 as u32);
    if options.is_none() {
        return;
    }
    let options = options.unwrap();

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
    let mut c: i32 = 0;
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

        // change terminal color
        if options.color {
            if c > 359 {
                c = 0;
            }

            let (r, g, b) = hsl::HSL {
                h: c as f64,
                s: 1_f64,
                l: 0.5_f64,
            }
            .to_rgb();
            c += 1;

            // just so you know, the code 48; sets the background color
            // requires TRUECOLOR terminal
            print!("\x1b[38;2;{r};{g};{b}m");
        }

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
