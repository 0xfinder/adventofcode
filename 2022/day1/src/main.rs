use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Error reading file");

    part_one(&input);

    part_two(&input);
}

fn part_one(input: &str) {
    let lines: Vec<&str> = input.split("\r\n").collect();

    let mut max = 0;
    let mut calories = 0;

    for line in lines {
        match line.parse::<i32>() {
            Ok(n) => calories += n,
            Err(_) => match calories > max {
                true => {
                    max = calories;
                    calories = 0;
                }
                false => calories = 0,
            },
        }
    }

    println!("{:?}", max);
}

fn part_two(input: &str) {
    let lines: Vec<&str> = input.split("\r\n").collect();

    let mut elves: [i32; 3] = [0; 3];

    let mut calories = 0;

    for line in lines {
        match line.parse::<i32>() {
            Ok(n) => calories += n,
            Err(_) => {
                if calories > elves[2] && calories < elves[1] {
                    elves[2] = calories;
                } else if calories > elves[1] && calories < elves[0] {
                    elves[2] = elves[1];
                    elves[1] = calories;
                } else if calories > elves[0] {
                    elves[2] = elves[1];
                    elves[1] = elves[0];
                    elves[0] = calories;
                }
                calories = 0;
            }
        }
    }

    let sum: i32 = elves.iter().sum();
    println!("{:?}", sum);
}
