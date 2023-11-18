fn main() {
    let weights = [16, 8, 9, 4, 3, 2, 4, 7, 7, 12, 3, 5, 4, 3, 2];
    let mut all_weigth = 0;
    for weight in weights {
        all_weigth += weight;
    }
    println!("all the boxes combined are {all_weigth} kgs");

    let mut boxes = Vec::new();
    let mut last_box = 0;
    for weight in weights {
        if (last_box + weight) > 20 {
            boxes.push(last_box);
            last_box = 0;
        }
        last_box += weight;
    }
    boxes.push(last_box);

    let mut box_4_string: String = "".to_string();
    for boxx in boxes {
        box_4_string += &boxx.to_string();
        box_4_string += " ";
    }
    println!("and the boxes we put them in should be {box_4_string}kgs");
}
