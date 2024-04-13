#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

// for time
use chrono::{prelude::*, TimeDelta};

// for terminal
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};
use crossterm::cursor::{Hide, Show};
use std::io::stdout;
#[macro_use]
extern crate crossterm;

// for map
use std::collections::HashMap;
use std::vec;
use lazy_static::lazy_static;

// for 2d arrays
// https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
use ndarray::Array2;

// ctrl+c exit
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// https://stackoverflow.com/questions/60273064/rust-best-practices-when-specifying-a-constant-hash-map
lazy_static! {
    static ref MAP: HashMap<&'static str, Vec<&'static str>> = {
        let map = HashMap::from([
            ("it is",  vec!["0-0", "0-1", "0-3", "0-4"]),
            ("01h",  vec!["5-0", "5-1", "5-2"]),
            ("02h",  vec!["6-8", "6-9", "6-10"]),
            ("03h",  vec!["5-6", "5-7", "5-8", "5-9", "5-10"]),
            ("04h",  vec!["6-0", "6-1", "6-2", "6-3"]),
            ("05h",  vec!["6-4", "6-5", "6-6", "6-7"]),
            ("06h",  vec!["5-3", "5-4", "5-5"]),
            ("07h",  vec!["8-0", "8-1", "8-2", "8-3", "8-4"]),
            ("08h",  vec!["7-0", "7-1", "7-2", "7-3", "7-4"]),
            ("09h",  vec!["4-7", "4-8", "4-9", "4-10"]),
            ("10h",  vec!["9-0", "9-1", "9-2"]),
            ("11h",  vec!["7-5", "7-6", "7-7", "7-8", "7-9", "7-10"]),
            ("00h",  vec!["8-5", "8-6", "8-7", "8-8", "8-9", "8-10"]),
            ("00m",  vec!["9-5", "9-6", "9-7", "9-8", "9-9", "9-10"]), // o'clock
            ("05m",  vec!["2-6", "2-7", "2-8", "2-9"]), // these numbers will be both 'from' & 'to'
            ("10m",  vec!["3-5", "3-6", "3-7"]),
            ("15m",  vec!["1-0", "1-2", "1-3", "1-4", "1-5", "1-6", "1-7", "1-8"]),
            ("20m",  vec!["2-0", "2-1", "2-2", "2-3", "2-4", "2-5"]),
            ("25m",  vec!["2-0", "2-1", "2-2", "2-3", "2-4", "2-5", "2-6", "2-7", "2-8", "2-9"]),
            ("30m",  vec!["3-0", "3-1", "3-2", "3-3"]), // half
            ("past",  vec!["4-0", "4-1", "4-2", "4-3"]),
            ("to",  vec!["3-9", "3-10"]),
        ]);
        map
    };
    static ref FACE: Array2<char> = {
        let v = vec![
            "itlisasampm",
            "acquarterdc",
            "twentyfivex",
            "halfstenfto",
            "pasterunine",
            "onesixthree",
            "fourfivetwo",
            "eighteleven",
            "seventwelve",
            "tenseoclock"
        ];
        // idk why didnt this work i mean it is literally what it wants innit
        // let face = Array2::from(v.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>());
        // so this is copilots answer; at least it works
        let face = Array2::from_shape_fn((10, 11), |(i, j)| v[i].chars().nth(j).unwrap());
        face
    };
}

fn convert_coordinates(string: &str) -> (usize, usize) {
    let mut iter = string.split("-");
    let x = iter.next().unwrap().parse::<usize>().unwrap();
    let y = iter.last().unwrap().parse::<usize>().unwrap();
    (x, y)
}

fn get_active_cells(hour: u8, min: u8) -> Array2<i8> {
    // since we cant have an empty array, and there is no good way to fill this up with booleans, we're using integers
    let mut toggle = Array2::zeros((11, 11));

    let mut to_toggle: Vec<&str> = vec![];
    let now = Local::now();

    to_toggle.extend_from_slice(MAP.get("it is").unwrap());

    //* minute
    let minute = now.minute();
    // we have 5 minute intervals and after thats done it switches
    let mut relative_min = minute - minute%5;
    // switches at relative 2:00 minutes instead of 2:30 to make it easier for me
    if minute%5 > 1 { relative_min+=5; } 
    // halftime is a special case
    if relative_min != 30 { relative_min = relative_min%30; }
    // after half time we invert the relative minutes ("past" -> "to")
    if minute > 31 { relative_min = 30 - relative_min; }
    if minute > 31 && relative_min == 30 { relative_min = 0; }
    // we get a string from it to get the correct one from the map
    let mut minute_string: String = format!("{}m", relative_min);
    if relative_min < 10 { minute_string = format!("0{}", minute_string); }
    to_toggle.extend_from_slice(MAP.get(minute_string.as_str()).unwrap());
    // console.log(minute, relative_min, minuteString);

    // "past" or "to"
    if minute <= 31 && relative_min != 0 { to_toggle.extend_from_slice(MAP.get("past").unwrap()); }
    else if minute > 31 && relative_min != 0 { to_toggle.extend_from_slice(MAP.get("to").unwrap()); }

    //* hour
    let hour = now.hour();
    let mut relative_hour = hour;
    // we need to add one when it switches from "past" to "to"
    if minute > 31 { relative_hour+=1; }
    // we only go up to 12, but basically we use american time. 0 = 12.
    relative_hour = relative_hour%12;
    // we get a string from it ...
    let mut hour_string: String = format!("{}h", relative_hour);
    if relative_hour < 10 { hour_string = format!("0{}", hour_string)}
    to_toggle.extend_from_slice(MAP.get(hour_string.as_str()).unwrap());

    // println!("{:?}", to_toggle);

    for i in to_toggle {
        let (x, y) = convert_coordinates(i);
        toggle[[x, y]] = 1;
    }

    toggle
}

fn display_time() {
    println!("{}", Local::now().format("%H:%M"));
    let toggle = get_active_cells(Local::now().hour() as u8, Local::now().minute() as u8);
    // println!();
    for i in 0..10 {
        for j in 0..11 {
            if toggle[[i, j]] == 1 {
                // https://stackoverflow.com/a/16267760/12706133
                print!("{}", FACE[[i, j]]);
            } else {
                print!(" ");
            }
            // print!("  ")
        }
        println!();
        // println!();
    }
}

fn main() -> std::io::Result<()> {
    // the ctrl+c handling was written by chatgpt

    // Switch to the alternate screen
    // enable_raw_mode().expect("Failed to enable raw mode");
    // execute!(std::io::stdout(), EnterAlternateScreen, Hide).expect("Failed to enter alternate screen");

    // Set up a flag to indicate whether Ctrl+C was pressed
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Handle SIGINT (Ctrl+C)
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");


    // local variable setup so that we can run stuff every minute
    let local = Local::now();
    // we get local time, add a minute, and then subtract the seconds and milliseconds
    // there is no good way to subtract milliseconds so this is a workaround
    let mut call_time = local.clone()
    .checked_add_signed(TimeDelta::try_minutes(1).unwrap()).unwrap()
    .checked_sub_signed(TimeDelta::try_seconds(local.second() as i64).unwrap()).unwrap()
    .checked_sub_signed(TimeDelta::try_milliseconds(local.clone().checked_sub_signed(TimeDelta::try_seconds(local.timestamp()).unwrap()).unwrap().timestamp_millis()).unwrap()).unwrap();

    // initial print
    display_time();
    //* main loop
    while running.load(Ordering::SeqCst) {
        println!("{}", Local::now().format("%H:%M %S:%3f"));
        if Local::now() >= call_time {
            // this can run every minute
            // clearing screen
            // execute!(stdout(), Clear(ClearType::Purge)).unwrap();

            display_time();

            // changing time to the next minute
            call_time = call_time.checked_add_signed(TimeDelta::try_minutes(1).unwrap()).unwrap();
            println!("calltime: {}", call_time);
        }
        // slows down main loop. it is higher duration than specified.
        // also ctrl+c only works when the sleep is over so we cant do minute long sleeps
        // https://users.rust-lang.org/t/how-do-you-make-an-infinite-loop-in-rust-and-make-it-run-with-a-delay/80296
        // we're just gonna calculate the current millisecond offset and subtract it from 1000
        let millis = 1000 - Local::now().timestamp_millis() % 1000;
        std::thread::sleep(std::time::Duration::from_millis(millis as u64));
    }

    // Leave the alternate screen before exiting
    // execute!(std::io::stdout(), LeaveAlternateScreen, Show).expect("Failed to leave alternate screen");
    // disable_raw_mode().expect("Failed to disable raw mode");
    Ok(())
}