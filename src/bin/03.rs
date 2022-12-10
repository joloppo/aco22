use advent_of_code::{ANSI_ITALIC, ANSI_RESET};
use std::{collections::HashMap, time::Instant};

fn get_alphabet_map() -> HashMap<char, u8> {
    let mut char_to_value = HashMap::<char, u8>::new();
    let chars = ('a'..='z').chain('A'..='Z');
    chars.enumerate().for_each(|(i, x)| {
        char_to_value.insert(x, (i + 1) as u8);
    });
    return char_to_value;
}
pub fn get_priority_value(line: &str, hashmap: &HashMap<char, u8>) -> u32 {
    let half_1 = &line[..(line.len() / 2)];
    let half_2 = &line[(line.len() / 2)..];
    let filtered: Vec<char> = half_1
        .chars()
        .filter(|c1| half_2.chars().any(|c2| c2 == *c1))
        .collect();
    *hashmap.get(&filtered[0]).unwrap() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let char_to_value = get_alphabet_map();
    let result = input
        .lines()
        .map(|s: &str| get_priority_value(s, &char_to_value))
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_to_value = get_alphabet_map();
    let mut lines = input.lines().peekable();
    let mut result: u32 = 0;
    while lines.peek().is_some() {
        let elf_1 = lines.next().unwrap();
        let elf_2 = lines.next().unwrap();
        let elf_3 = lines.next().unwrap();
        let filtered: Vec<char> = elf_1
            .chars()
            .filter(|c1| elf_2.chars().any(|c2| c2 == *c1))
            .filter(|c2| elf_3.chars().any(|c3| c3 == *c2))
            .collect();
        result += *char_to_value.get(&filtered[0]).unwrap() as u32
    }
    Some(result)
}

fn main() {
    let timer = Instant::now();
    let input = &advent_of_code::read_file("inputs", 3);
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
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
