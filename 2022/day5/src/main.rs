use std::fs;

fn main() {
    part_one();

    part_two();
}

fn part_one() {
    let input = fs::read_to_string("input.txt").expect("error reading file");

    let (starting, procedure) = input.split_once("\r\n\r\n").unwrap();
    let (starting, stack_numbers) = starting.rsplit_once("\n").unwrap();

    let mut stacks: Vec<Vec<char>> =
        vec![Vec::new(); stack_numbers.split_whitespace().collect::<Vec<_>>().len()];

    for line in starting.lines().rev() {
        let mut prev: Option<char> = None;
        for (idx, c) in line.chars().enumerate() {
            if prev == Some('[') {
                stacks[(idx - 1) / 4].push(c)
            }
            prev = Some(c);
        }
    }

    for line in procedure.lines() {
        let instruction = line
            .split(" ")
            .enumerate()
            .filter(|&(i, _)| i % 2 == 1)
            .map(|(_, e)| e)
            .collect::<Vec<_>>();

        let number = instruction[0].parse::<usize>().unwrap();
        let from = instruction[1].parse::<usize>().unwrap();
        let to = instruction[2].parse::<usize>().unwrap();

        for _ in 0..number {
            let removed = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(removed)
        }
    }

    let mut top = "".to_string();
    for stack in stacks {
        top.push(*stack.last().unwrap())
    }
    println!("{}", top);
}

fn part_two() {
    let input = fs::read_to_string("input.txt").expect("error reading file");

    let (starting, procedure) = input.split_once("\r\n\r\n").unwrap();
    let (starting, stack_numbers) = starting.rsplit_once("\n").unwrap();

    let mut stacks: Vec<Vec<char>> =
        vec![Vec::new(); stack_numbers.split_whitespace().collect::<Vec<_>>().len()];

    for line in starting.lines().rev() {
        let mut prev: Option<char> = None;
        for (idx, c) in line.chars().enumerate() {
            if prev == Some('[') {
                stacks[(idx - 1) / 4].push(c)
            }
            prev = Some(c);
        }
    }

    for line in procedure.lines() {
        let instruction = line
            .split(" ")
            .enumerate()
            .filter(|&(i, _)| i % 2 == 1)
            .map(|(_, e)| e)
            .collect::<Vec<_>>();

        let number = instruction[0].parse::<usize>().unwrap();
        let from = instruction[1].parse::<usize>().unwrap();
        let to = instruction[2].parse::<usize>().unwrap();

        let mut v: Vec<char> = Vec::new();
        for _ in 0..number {
            v.push(stacks[from - 1].pop().unwrap());
        }

        for _ in 0..v.len() {
            stacks[to - 1].push(v.pop().unwrap());
        }
    }

    let mut top = "".to_string();
    for stack in stacks {
        top.push(*stack.last().unwrap())
    }
    println!("{}", top);
}
