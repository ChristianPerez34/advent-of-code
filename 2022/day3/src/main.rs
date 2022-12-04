use std::fs;

fn determine_priority(main_chunk: &str, remaining_chunks: Vec<&str>) -> u32 {
    let common_item = main_chunk
        .chars()
        .find(|element| remaining_chunks.iter().all(|item| item.contains(*element)))
        .unwrap();
    let ascii_number = common_item as u32;
    let priority = match common_item.is_ascii_lowercase() {
        true => &ascii_number - 96,
        false => &ascii_number - 38,
    };
    priority
}

fn process_part1(input_text: &String) -> u32 {
    input_text
        .lines()
        .map(|line| {
            let (first_half, second_half) = line.split_at(line.len() / 2);
            determine_priority(first_half, vec![second_half])
        })
        .sum::<u32>()
}

fn process_part2(input_text: &String) -> u32 {
    input_text
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| determine_priority(chunk[0], chunk[1..].to_vec()))
        .sum::<u32>()
}

fn main() {
    let input_text = fs::read_to_string("input\\input.txt").unwrap();
    println!("{}", process_part1(&input_text));
    println!("{}", process_part2(&input_text));
}
