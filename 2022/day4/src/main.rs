use std::{fs, ops::RangeInclusive};

fn is_fully_contained(
    first_section: RangeInclusive<i32>,
    second_section: RangeInclusive<i32>,
) -> bool {
    (first_section.start() >= second_section.start() && first_section.end() <= second_section.end())
        || (second_section.start() >= first_section.start()
            && second_section.end() <= first_section.end())
}

fn is_overlapping(first_section: RangeInclusive<i32>, second_section: RangeInclusive<i32>) -> bool {
    first_section
        .clone()
        .any(|item| second_section.contains(&item))
}

fn process_part1(input_text: &String) -> i32 {
    input_text
        .lines()
        .map(|line| {
            let sections_ids = line
                .split(",")
                .map(|section| {
                    let ids: Vec<i32> = section
                        .split("-")
                        .map(|item| item.parse::<i32>().unwrap())
                        .collect();
                    ids[0]..=ids[1]
                })
                .collect::<Vec<RangeInclusive<i32>>>();
            match is_fully_contained(sections_ids[0].clone(), sections_ids[1].clone()) {
                true => 1,
                false => 0,
            }
        })
        .sum::<i32>()
}

fn process_part2(input_text: &String) -> i32 {
    input_text
        .lines()
        .map(|line| {
            let sections_ids = line
                .split(",")
                .map(|section| {
                    let ids: Vec<i32> = section
                        .split("-")
                        .map(|item| item.parse::<i32>().unwrap())
                        .collect();
                    ids[0]..=ids[1]
                })
                .collect::<Vec<RangeInclusive<i32>>>();
            match is_overlapping(sections_ids[0].clone(), sections_ids[1].clone()) {
                true => 1,
                false => 0,
            }
        })
        .sum::<i32>()
}

fn main() {
    let input_text = fs::read_to_string("input\\input.txt").unwrap();
    println!("{}", process_part1(&input_text));
    println!("{}", process_part2(&input_text));
}
