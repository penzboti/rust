use crossterm::terminal::size as termsize;
use std::collections::HashMap;

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

pub fn render(pointer: &crate::Pos, target: &crate::Pos) {
    let size = get_terminal_size();
    for y in 1..(size["rows"] + 1) {
        for x in 1..(size["cols"] + 1) {
            if x == pointer.x && y == pointer.y {
                print!("P");
            } else if x == target.x && y == target.y {
                print!("T");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}