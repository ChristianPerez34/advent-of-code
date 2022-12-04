use std::fs;

fn process_part1(input_text: &String) -> u32 {
    input_text
        .lines()
        .map(|line| {
            let (first_half, second_half) = line.split_at(line.len() / 2);
            let common_item = first_half
                .chars()
                .find(|element| second_half.contains(*element))
                .unwrap();
            let ascii_number = common_item as u32;
            let priority = match common_item.is_ascii_lowercase() {
                true => &ascii_number - 96,
                false => &ascii_number - 38,
            };
            priority
        })
        .sum::<u32>()
}

fn process_part2(input_text: &String) -> u32 {
    input_text
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let common_item = chunk[0]
                .chars()
                .find(|element| chunk[1].contains(*element) & chunk[2].contains(*element))
                .unwrap();
            let ascii_number = common_item as u32;
            let priority = match common_item.is_ascii_lowercase() {
                true => &ascii_number - 96,
                false => &ascii_number - 38,
            };
            priority
        })
        .sum::<u32>()
}

fn main() {
    let input_text = fs::read_to_string("input\\input.txt").unwrap();
    println!("{}", process_part1(&input_text));
    println!("{}", process_part2(&input_text));
}
