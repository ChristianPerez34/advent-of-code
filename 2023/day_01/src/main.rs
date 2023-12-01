use std::fs;

/// Processes a line of text, extracting the first and last numeric characters, if any, and
/// concatenating them into a single integer. If no numeric characters are found, returns 0.
///
/// # Arguments
///
/// * `line` - A string slice that holds the line of text to be processed.
///
/// # Returns
///
/// * An i32 integer composed of the first and last numeric characters found in the line.
///   If no numeric characters are found, returns 0.
fn process_line(line: &str) -> i32 {
    let numbers = line
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>();

    match numbers.is_empty() {
        true => 0,
        false => format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<i32>()
            .expect("Unable to convert to i32"),
    }
}

fn part_1(input: &str) -> i32 {
    input.lines().map(process_line).sum()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut modified_line = String::from("");
            for index in 0..line.len() {
                let reduced_line = &line[index..];
                let character = if reduced_line.starts_with("one") {
                    "1".to_string()
                } else if reduced_line.starts_with("two") {
                    "2".to_string()
                } else if reduced_line.starts_with("three") {
                    "3".to_string()
                } else if reduced_line.starts_with("four") {
                    "4".to_string()
                } else if reduced_line.starts_with("five") {
                    "5".to_string()
                } else if reduced_line.starts_with("six") {
                    "6".to_string()
                } else if reduced_line.starts_with("seven") {
                    "7".to_string()
                } else if reduced_line.starts_with("eight") {
                    "8".to_string()
                } else if reduced_line.starts_with("nine") {
                    "9".to_string()
                } else {
                    let first_char = reduced_line
                        .chars()
                        .next()
                        .expect("Unable to get character")
                        .to_string();
                    first_char
                };
                modified_line = modified_line + &character;
            }
            return process_line(&modified_line);
        })
        .sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/test_case.txt").expect("Unable to read file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
