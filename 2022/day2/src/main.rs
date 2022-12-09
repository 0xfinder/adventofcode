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

fn part_two() {
    let file = fs::File::open("input.txt").expect("File not found");
    let reader = io::BufReader::new(file);

    let mut score = 0;

    // A for Rock, B for Paper, and C for Scissors
    // 1 2 3
    // X Y Z
    // lose draw win
    let mut shape = "";

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let (opponent, me) = line.split_once(" ").unwrap();

        let outcome_score = match (opponent, me) {
            ("A", "X") | ("B", "X") | ("C", "X") => {
                shape = match opponent {
                    "A" => "Z",
                    "B" => "X",
                    "C" => "Y",
                    _ => "",
                };
                0
            }
            ("A", "Y") | ("B", "Y") | ("C", "Y") => {
                shape = match opponent {
                    "A" => "X",
                    "B" => "Y",
                    "C" => "Z",
                    _ => "",
                };
                3
            }
            ("A", "Z") | ("B", "Z") | ("C", "Z") => {
                shape = match opponent {
                    "A" => "Y",
                    "B" => "Z",
                    "C" => "X",
                    _ => "",
                };
                6
            }
            _ => 0,
        };

        let shape_score = match shape {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        score += outcome_score + shape_score;
    }
    println!("{:?}", score);
}
