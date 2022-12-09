use std::fs;

fn main() {
    println!("{:?}", part_one());

    println!("{:?}", part_two());
}

fn part_one() -> i32 {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.split("\r\n").collect();

    let mut count = 0;

    for line in lines {
        let v: Vec<_> = line.split(&[',', '-'][..]).collect();
        let v: Vec<i32> = v.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        if (v[0] <= v[2] && v[1] >= v[3]) || (v[0] >= v[2] && v[1] <= v[3]) {
            count += 1;
        }
    }

    count
}
fn part_two() -> usize {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    input
        .lines()
        .filter(|line| {
            let v: Vec<_> = line.split(&[',', '-'][..]).collect();
            let v: Vec<i32> = v.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
            v[0] <= v[3] && v[1] >= v[2]
        })
        .count()
}
