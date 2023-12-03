use std::{collections::HashMap, fs};

/// Checks if a game is possible based on the color and quantity of cubes.
///
/// # Arguments
///
/// * `color` - A string slice that holds the color of the cube. It can be "red", "green", or "blue".
/// * `quantity` - A u32 that holds the quantity of the cubes. The maximum quantity for each color is as follows:
///     * "red" => 12
///     * "green" => 13
///     * "blue" => 14
///
/// # Returns
///
/// * `bool` - Returns true if the game is possible (i.e., the quantity does not exceed the maximum for the given color), false otherwise.
///
/// # Example
///
/// ```
/// let is_possible = is_game_possible("red", 10);
/// assert_eq!(is_possible, true);
/// ```
fn is_game_possible(color: &str, quantity: u32) -> bool {
    match color {
        "red" => quantity <= 12,
        "green" => quantity <= 13,
        "blue" => quantity <= 14,
        _ => false,
    }
}

fn part_01(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let modified_line = &line[line.find(":").unwrap() + 1..];
            let game_possible = modified_line.split(";").all(|set| {
                set.split(",").all(|roll| {
                    let cube = roll.trim().split(" ").collect::<Vec<&str>>();
                    is_game_possible(
                        cube[1],
                        cube[0]
                            .parse::<u32>()
                            .expect("Unable to parse string as number"),
                    )
                })
            });
            if game_possible {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_02(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let modified_line = &line[line.find(":").unwrap() + 1..];
            let mut game = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);

            for set in modified_line.split(';') {
                for roll in set.split(',') {
                    let cube = roll.trim().split_whitespace().collect::<Vec<&str>>();
                    let color = cube[1];
                    let number = cube[0].parse::<i32>().unwrap();

                    game.entry(color).and_modify(|e| *e = (*e).max(number));
                }
            }
            game.get("red").unwrap() * game.get("green").unwrap() * game.get("blue").unwrap()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("inputs/part_1.txt").expect("Unable to read file");
    println!("Part 1: {}", part_01(&input));
    println!("Part 2: {}", part_02(&input))
    // part_01(&part_01_contents);
}
