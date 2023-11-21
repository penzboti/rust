use crossterm::terminal::size as termsize;
use std::collections::HashMap;
// get terminal size with crossterm
fn get_terminal_size() -> HashMap<String, u16>{
    // ty for showing me hashmaps
    // https://stackoverflow.com/a/55376723/12706133
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
