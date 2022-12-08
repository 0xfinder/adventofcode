use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let file = fs::File::open("input.txt").expect("File not found");
    let reader = io::BufReader::new(file);

    let alphabet = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priorities = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let (c1, c2) = line.split_at(line.len() / 2);
        for c in c1.chars() {
            if c2.contains(c) {
                priorities += alphabet.find(c).unwrap();
                break;
            }
        }
    }

    println!("{}", priorities);
}
