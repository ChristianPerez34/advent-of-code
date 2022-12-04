use std::fs;

fn determine_elf_calories(input_text: String) -> Vec<i32> {
    input_text
        .split("\r\n\r\n")
        .map(|lines| {
            lines
                .split_terminator("\r\n")
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>()
}

fn main() {
    let input_text = fs::read_to_string("input\\input.txt").unwrap();
    let mut elf_calories = determine_elf_calories(input_text);
    let max = elf_calories.iter().max().unwrap();

    println!("Part 1: {}", max);

    elf_calories.sort_by(|a, b| b.cmp(a));
    let top_3 = &elf_calories.iter().take(3).sum::<i32>();

    println!("Part 2: {}", top_3);
}
