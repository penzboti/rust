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

// for 2d arrays
// https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
use ndarray::Array2;

// ctrl+c exit
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn get_active_cells(hour: u8, min: u8) -> Array2<i8> {
    // since we cant have an empty array, and there is no good way to fill this up with booleans, we're using integers
    let mut toggle = Array2::zeros((11, 10));
    // just testing over here
    toggle[[0, 0]] = 1;
    toggle
}

fn display_time() {
    println!("{}", Local::now().format("%H:%M"));
    let toggle = get_active_cells(Local::now().hour() as u8, Local::now().minute() as u8);
    for i in 0..11 {
        for j in 0..10 {
            if toggle[[i, j]] == 1 {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!();
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
        println!("{} {}", Local::now(), call_time);
        if Local::now() >= call_time {
            // this can run every minute
            // clearing screen
            // execute!(stdout(), Clear(ClearType::Purge)).unwrap();

            display_time();

            // changing time to the next minute
            call_time = call_time.checked_add_signed(TimeDelta::try_minutes(1).unwrap()).unwrap();
        }
        // slows down main loop. it is higher duration than specified.
        // also ctrl+c only works when the sleep is over so we cant do minute long sleeps
        // https://users.rust-lang.org/t/how-do-you-make-an-infinite-loop-in-rust-and-make-it-run-with-a-delay/80296
        //TODO make a second syncer here: sometimes 900ms and sometimes 1s sleep
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // Leave the alternate screen before exiting
    // execute!(std::io::stdout(), LeaveAlternateScreen, Show).expect("Failed to leave alternate screen");
    // disable_raw_mode().expect("Failed to disable raw mode");
    Ok(())
}