use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Error reading file");

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
