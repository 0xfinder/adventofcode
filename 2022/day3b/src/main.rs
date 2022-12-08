use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.split("\r\n").collect();

    let alphabet = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priorities = 0;

    for i in (0..lines.len()).step_by(3) {
        let group = &lines[i..i + 3];
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                priorities += alphabet.find(c).unwrap();
                break;
            }
        }
    }

    println!("{}", priorities);
}
