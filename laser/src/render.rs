use crossterm::terminal::size as termsize;
use std::collections::HashMap;
use crate::publicvars::{P_XPOS, P_YPOS, T_XPOS, T_YPOS, update_var};
use std::sync::atomic::Ordering;

// get terminal size with crossterm
pub fn get_terminal_size() -> HashMap<String, u16>{
    // ty for showing me hashmaps
    // https://stackoverflow.com/a/55376723/12706133
    // cool hashmap tutorial
    // https://www.koderhq.com/tutorial/rust/hashmap/
    let mut size: HashMap<String, u16> = HashMap::new();
    // how to get a value form a result
    // https://stackoverflow.com/questions/63859927/how-to-get-a-value-from-a-result
    match termsize() {
        Ok(size_tup) => {
            size.insert("cols".to_string(), size_tup.0);
            size.insert("rows".to_string(), size_tup.1);
        },
        Err(e) => eprintln!("{:?}", e),
    }
    size
}

pub fn render() {
    for x in 1..(get_terminal_size()["rows"] + 1) {
        for j in 1..(get_terminal_size()["cols"] + 1) {
            if x == P_YPOS.load(Ordering::Relaxed) && j == P_XPOS.load(Ordering::Relaxed) {
                print!("P");
            } else if x == T_YPOS.load(Ordering::Relaxed) && j == T_XPOS.load(Ordering::Relaxed) {
                print!("T");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    // crate::publicvars::print_all_vars();
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