use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let file = fs::File::open("input.txt").expect("File not found");
    let reader = io::BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let (opponent, me) = line.split_once(" ").unwrap();

        let outcome_score = match (opponent, me) {
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => 0,
        };

        let shape_score = match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        score += outcome_score + shape_score;
    }
    println!("{:?}", score);
}
