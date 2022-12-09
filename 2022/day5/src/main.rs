use std::{collections::VecDeque, fs};

struct Move {
    amount: i32,
    from: i32,
    to: i32,
}

fn process_part1(input_lines: &Vec<&str>) -> String {
    let stack_num = input_lines[0].len() / 4 + 1;
    let mut stack: Vec<VecDeque<char>> = vec![VecDeque::new(); stack_num];

    for line in input_lines {
        if line.contains("move") {
            let action: Vec<&str> = line.split(" ").collect();
            let movement = Move {
                amount: action[1].parse::<i32>().unwrap(),
                from: action[3].parse::<i32>().unwrap(),
                to: action[5].parse::<i32>().unwrap(),
            };
            for _ in 0..movement.amount {
                let value = stack[(movement.from - 1) as usize].pop_front().unwrap();
                stack[(movement.to - 1) as usize].push_front(value);
            }
        } else if !line.is_empty() {
            line.as_bytes()
                .chunks(4)
                .enumerate()
                .for_each(|(index, chunk)| {
                    let letter = chunk[1] as char;
                    if letter.is_alphabetic() {
                        stack[index].push_back(letter);
                    }
                });
        }
    }
    stack
        .iter()
        .map(|s| s[0].to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn process_part2(input_lines: &Vec<&str>) -> String {
    let stack_num = input_lines[0].len() / 4 + 1;
    let mut stack: Vec<VecDeque<char>> = vec![VecDeque::new(); stack_num];

    for line in input_lines {
        if line.contains("move") {
            let action: Vec<&str> = line.split(" ").collect();
            let movement = Move {
                amount: action[1].parse::<i32>().unwrap(),
                from: action[3].parse::<i32>().unwrap(),
                to: action[5].parse::<i32>().unwrap(),
            };
            let values = stack[(movement.from - 1) as usize].split_off(movement.amount as usize);

            stack[(movement.from - 1) as usize]
                .clone()
                .iter()
                .rev()
                .for_each(|letter| {
                    stack[(movement.to - 1) as usize].push_front(*letter);
                });

            stack[(movement.from - 1) as usize] = values;
        } else if !line.is_empty() {
            line.as_bytes()
                .chunks(4)
                .enumerate()
                .for_each(|(index, chunk)| {
                    let letter = chunk[1] as char;
                    if letter.is_alphabetic() {
                        stack[index].push_back(letter);
                    }
                });
        }
    }
    stack
        .iter()
        .map(|s| s[0].to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    let input_text = fs::read_to_string("input/input.txt").unwrap();
    let input_lines: Vec<&str> = input_text.lines().collect();
    println!("{}", process_part1(&input_lines));
    println!("{}", process_part2(&input_lines));
}
