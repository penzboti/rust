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

// chatgpt. uses an algorithm of course. the Bresenham's Line Algorithm.
fn draw_line(array: &mut Vec<Vec<u32>>, x0: usize, y0: usize, x1: usize, y1: usize) {
    let dx = (x1 as isize - x0 as isize).abs();
    let dy = (y1 as isize - y0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;

    while x < array[0].len() && y < array.len() {
        array[y][x] = 1;

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x = (x as isize + sx) as usize;
        }
        if e2 < dx {
            err += dx;
            y = (y as isize + sy) as usize;
        }
    }
}

pub fn render(pointer: &crate::Pos, target: &crate::Pos) {
    let size = get_terminal_size();

    let x0 = pointer.x as usize;
    let x1 = target.x as usize;
    let y0 = pointer.y as usize;
    let y1 = target.y as usize;
    let array_width = size["cols"] as usize;
    let array_height = size["rows"] as usize;
    let mut arr = vec![vec![0; array_width]; array_height];

    if x0 != x1 || y0 != y1 {
        draw_line(&mut arr, x0, y0, x1, y1);
    }

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if i == y0 && j == x0 {
                print!("P");
            } else
            if i == y1 && j == x1 {
                print!("T");
            } else
            if arr[i][j] == 1 {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }

}