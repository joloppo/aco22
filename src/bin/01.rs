pub fn part_one(input: &str) -> Option<u32> {
    let u32_vec: Vec<u32> = input
        .lines()
        .map(|s: &str| {
            if s.is_empty() {
                0
            } else {
                s.parse::<u32>().unwrap()
            }
        })
        .collect::<Vec<u32>>();
    let mut max_elf_calories: u32 = 0;
    let mut current_calories: u32 = 0;
    for (ix, calorie_item) in u32_vec.iter().enumerate() {
        if calorie_item.clone() == 0 || (ix + 1 == input.len()) {
            if current_calories > max_elf_calories {
                max_elf_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += calorie_item;
        }
    }
    Some(max_elf_calories)
}

struct TopThree(u32, u32, u32);

impl TopThree {
    fn insert(&mut self, value: u32) {
        let mut interrim: u32 = value;
        if interrim > self.0 {
            (interrim, self.0) = (self.0, interrim);
        }
        if interrim > self.1 {
            (interrim, self.1) = (self.1, interrim);
        }
        if interrim > self.2 {
            self.2 = interrim;
        }
    }
    fn sum(self) -> u32 {
        self.0 + self.1 + self.2
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let u32_vec: Vec<u32> = input
        .lines()
        .map(|s: &str| {
            if s.is_empty() {
                0
            } else {
                s.parse::<u32>().unwrap()
            }
        })
        .collect::<Vec<u32>>();
    let mut current_calories: u32 = 0;
    let mut top_three = TopThree(0, 0, 0);
    for (ix, calorie_item) in u32_vec.iter().enumerate() {
        if calorie_item.clone() == 0 || (ix + 1 == input.len()) {
            top_three.insert(current_calories);
            current_calories = 0;
        } else {
            current_calories += calorie_item;
        }
    }
    Some(top_three.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
