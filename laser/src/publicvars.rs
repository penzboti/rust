use std::sync::atomic::{AtomicU16, AtomicBool, Ordering};

// public mutable variables
// https://morestina.net/blog/1774/rust-global-variables-demystified
pub static WASDMODE: AtomicBool = AtomicBool::new(true);
pub static P_XPOS: AtomicU16 = AtomicU16::new(1);
pub static P_YPOS: AtomicU16 = AtomicU16::new(1);
pub static T_XPOS: AtomicU16 = AtomicU16::new(1);
pub static T_YPOS: AtomicU16 = AtomicU16::new(1);

// initial help with signed and unsigned integers
// https://stackoverflow.com/a/73153135/12706133

// i love pointers after this function i came up with on my own
pub fn update_var(name: &AtomicU16, diff: i16, axis: char) -> bool {
    let ptr: i16 = name.load(Ordering::Relaxed) as i16;
    let size = crate::render::get_terminal_size();
    let iresult = ptr + diff;

    if iresult >= 1 {
        let uresult: u16 = iresult as u16;
        if axis == 'x' && uresult <= size["cols"] || axis == 'y' && uresult <= size["rows"] {
            name.store(uresult, Ordering::Relaxed);
            return true;
        }
    }
    
    false
}

pub fn print_all_vars() {
    let pointer_xpos = P_XPOS.load(Ordering::Relaxed);
    let pointer_ypos = P_YPOS.load(Ordering::Relaxed);
    let target_xpos = T_XPOS.load(Ordering::Relaxed);
    let target_ypos = T_YPOS.load(Ordering::Relaxed);
    println!("p: {}, {}, t: {}, {}", pointer_xpos, pointer_ypos, target_xpos, target_ypos);
}