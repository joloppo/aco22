use std::time::Instant;
use advent_of_code::{ANSI_ITALIC, ANSI_RESET};

#[derive(PartialEq, Eq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq)]

enum OutCome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn map_opponent_to_hand(s: &str) -> Hand {
    match s.chars().nth(0).unwrap() {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        'C' => Hand::Scissors,
        _ => Hand::Scissors,
    }
}
fn map_myself_to_hand(s: &str) -> Hand {
    match s.chars().nth(2).unwrap() {
        'X' => Hand::Rock,
        'Y' => Hand::Paper,
        'Z' => Hand::Scissors,
        _ => Hand::Scissors,
    }
}

fn map_myself_to_outcome(s: &str) -> OutCome {
    match s.chars().nth(2).unwrap() {
        'X' => OutCome::Lose,
        'Y' => OutCome::Draw,
        'Z' => OutCome::Win,
        _ => OutCome::Lose,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|s: &str| -> u32 {
            let mut score = 0;
            let opponent_hand = map_opponent_to_hand(s);
            let my_hand = map_myself_to_hand(s);
            if opponent_hand == my_hand {
                score += OutCome::Draw as u32;
            } else if opponent_hand == Hand::Rock && my_hand == Hand::Paper
                || opponent_hand == Hand::Paper && my_hand == Hand::Scissors
                || opponent_hand == Hand::Scissors && my_hand == Hand::Rock
            {
                score += OutCome::Win as u32;
            };
            score += my_hand as u32;
            score
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|s: &str| -> u32 {
            let mut score = 0;
            let opponent_hand = map_opponent_to_hand(s);
            let outcome = map_myself_to_outcome(s);
            if outcome == OutCome::Draw {
                score += opponent_hand as u32;
            } else if outcome == OutCome::Win {
                let my_hand = match opponent_hand {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock,
                };
                score += my_hand as u32;
            } else if outcome == OutCome::Lose {
                let my_hand = match opponent_hand {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper,
                };
                score += my_hand as u32;
            }
            score += outcome as u32;
            score
        })
        .sum::<u32>();
    Some(result)
}

fn main() {
    let timer = Instant::now();
    let input = &advent_of_code::read_file("inputs", 2);
    let elapsed = timer.elapsed();
    println!(
        "{} {}(elapsed: {:.2?}){}",
        "File load", ANSI_ITALIC, elapsed, ANSI_RESET
    );
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
