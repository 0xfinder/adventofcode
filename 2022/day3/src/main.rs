use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    part_one();

    part_two();
}

fn part_one() {
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
fn part_two() {
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
