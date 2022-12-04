use std::{fs, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

struct Play {
    opponent: Hand,
    player: Hand,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissor),
            _ => Err("Invalid play".to_string()),
        }
    }
}

fn match_round_result(play: &Play) -> i32 {
    let round_score = match play {
        Play {
            opponent: Hand::Rock,
            player: Hand::Scissor,
        }
        | Play {
            opponent: Hand::Paper,
            player: Hand::Rock,
        }
        | Play {
            opponent: Hand::Scissor,
            player: Hand::Paper,
        } => 0,
        Play {
            opponent: Hand::Rock,
            player: Hand::Rock,
        }
        | Play {
            opponent: Hand::Paper,
            player: Hand::Paper,
        }
        | Play {
            opponent: Hand::Scissor,
            player: Hand::Scissor,
        } => 3,
        Play {
            opponent: Hand::Rock,
            player: Hand::Paper,
        }
        | Play {
            opponent: Hand::Paper,
            player: Hand::Scissor,
        }
        | Play {
            opponent: Hand::Scissor,
            player: Hand::Rock,
        } => 6,
    };
    round_score + play.player as i32
}

fn process_part1(input_text: &String) -> i32 {
    input_text
        .lines()
        .map(|line| {
            let hands: Vec<Hand> = line
                .split(" ")
                .map(|hand| hand.parse::<Hand>().unwrap())
                .collect();
            let play = Play {
                opponent: hands[0],
                player: hands[1],
            };
            match_round_result(&play)
        })
        .sum::<i32>()
}

fn process_part2(input_text: &String) -> i32 {
    input_text
        .lines()
        .map(|line| {
            let hands: Vec<&str> = line.split(" ").collect();
            let opponent = hands[0].parse::<Hand>().unwrap();
            let player = match hands[1] {
                "X" => match opponent {
                    Hand::Rock => Hand::Scissor,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissor => Hand::Paper,
                },
                "Y" => opponent,
                "Z" => match opponent {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissor,
                    Hand::Scissor => Hand::Rock,
                },
                _ => panic!("Unexpected input"),
            };
            let play = Play { opponent, player };
            match_round_result(&play)
        })
        .sum::<i32>()
}

fn main() {
    // Part 1
    let input_text = fs::read_to_string("input\\input.txt").unwrap();
    let mut score = process_part1(&input_text);
    println!("{:?}", score);

    //Part 2
    score = process_part2(&input_text);
    println!("{:?}", score);
}
