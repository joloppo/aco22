use advent_of_code::{ANSI_ITALIC, ANSI_RESET};
use std::{borrow::Borrow, time::Instant};

pub fn part_one(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");
    let initial_stacks_str = split.next().unwrap();
    let moves_str = split.next().unwrap();

    let lines: Vec<&str> = initial_stacks_str.lines().collect();

    let control_indices: Vec<usize> = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(idx, c)| if c == ' ' { 0 } else { idx })
        .filter(|idx| *idx != 0)
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in &control_indices {
        stacks.push(Vec::new())
    }

    for line in lines[..(lines.len() - 1)].iter().rev() {
        for (stack_idx, str_idx) in control_indices.iter().enumerate() {
            let select = line.chars().nth(*str_idx).unwrap();
            if select != ' ' {
                stacks[stack_idx].push(select);
            }
        }
    }

    for line in moves_str.lines() {
        let line_split: Vec<&str> = line.split(" ").collect();
        let move_times = line_split[1].parse::<usize>().unwrap();
        let move_source = line_split[3].parse::<usize>().unwrap() - 1;
        let move_destination = line_split[5].parse::<usize>().unwrap() - 1;
        for _ in 0..move_times {
            let value = &stacks[move_source].pop().unwrap();
            stacks[move_destination].push(*value);
        }
    }

    let result: String = stacks.iter().map(|v| v.last().unwrap()).collect();
    return Some(result);
}

pub fn part_two(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");
    let initial_stacks_str = split.next().unwrap();
    let moves_str = split.next().unwrap();

    let lines: Vec<&str> = initial_stacks_str.lines().collect();

    let control_indices: Vec<usize> = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(idx, c)| if c == ' ' { 0 } else { idx })
        .filter(|idx| *idx != 0)
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in &control_indices {
        stacks.push(Vec::new())
    }

    for line in lines[..(lines.len() - 1)].iter().rev() {
        for (stack_idx, str_idx) in control_indices.iter().enumerate() {
            let select = line.chars().nth(*str_idx).unwrap();
            if select != ' ' {
                stacks[stack_idx].push(select);
            }
        }
    }

    for line in moves_str.lines() {
        let line_split: Vec<&str> = line.split(" ").collect();
        let move_times = line_split[1].parse::<usize>().unwrap();
        let move_source = line_split[3].parse::<usize>().unwrap() - 1;
        let move_destination = line_split[5].parse::<usize>().unwrap() - 1;
        let mut crane_holder: Vec<char> = vec![];
        for _ in 0..move_times {
            let value = stacks[move_source].pop().unwrap();
            crane_holder.push(value);
        }
        for _crate in crane_holder.iter().rev() {
            stacks[move_destination].push(*_crate);
        }
    }

    let result: String = stacks.iter().map(|v| v.last().unwrap()).collect();
    return Some(result);
}

fn main() {
    let timer = Instant::now();
    let input = &advent_of_code::read_file("inputs", 5);
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
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
