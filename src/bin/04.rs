use advent_of_code::{ANSI_ITALIC, ANSI_RESET};
use std::time::Instant;

fn get_groups(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut groups = line.split(",");
    let group1: Vec<u32> = groups
        .next()
        .unwrap()
        .split("-")
        .map(|s: &str| s.parse::<u32>().unwrap())
        .collect();
    let group2: Vec<u32> = groups
        .next()
        .unwrap()
        .split("-")
        .map(|s: &str| s.parse::<u32>().unwrap())
        .collect();
    (group1, group2)
}

fn is_redundant(line: &str) -> bool {
    let groups = get_groups(line);
    if groups.0[0] <= groups.1[0] && groups.1[1] <= groups.0[1] {
        return true;
    }
    if groups.1[0] <= groups.0[0] && groups.0[1] <= groups.1[1] {
        return true;
    }
    return false;
}

fn is_redundant_overlap(line: &str) -> bool {
    let groups = get_groups(line);
    groups.0[0] <= groups.1[1] && groups.1[0] <= groups.0[1]
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line: &str| if is_redundant(line) { 1 } else { 0 })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line: &str| if is_redundant_overlap(line) { 1 } else { 0 })
        .sum::<u32>();
    Some(result)
}

fn main() {
    let timer = Instant::now();
    let input = &advent_of_code::read_file("inputs", 4);
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
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
