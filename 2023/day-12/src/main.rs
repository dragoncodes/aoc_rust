use itertools::*;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start_part_1 = Instant::now();
    let part_1_result = solution_part_1(&input);
    let part_1_time = start_part_1.elapsed();

    println!("Part 1: {} ({:?})", part_1_result, part_1_time);

    let input = std::fs::read_to_string("input.txt").unwrap();

    let start_part_2 = Instant::now();
    let part_2_result = solution_part_2(&input);
    let part_2_time = start_part_2.elapsed();

    println!("Part 2: {} ({:?})", part_2_result, part_2_time);
}

#[derive(Debug)]
struct Game {
    spaces_to_fill: u32,
    line: String,
    batches: Vec<u32>,
}

impl Game {
    fn generate_combinations(&self) -> Vec<String> {
        let options: Vec<String> = repeat_n([".", "#"].into_iter(), self.spaces_to_fill as usize)
            .multi_cartesian_product()
            .map(|v| v.join(""))
            .collect();

        options
    }

    fn check_option(&self, option: &str) -> bool {
        let mut option_iter = option.chars();
        let filled_option = self
            .line
            .chars()
            .map(|c| match c {
                '?' => option_iter.next().unwrap(),
                value => value,
            })
            .collect::<String>();

        let counts = filled_option
            .chars()
            .group_by(|c| c == &'#')
            .into_iter()
            .filter_map(|(is_hash, group)| is_hash.then_some(group.into_iter().count() as u32))
            .collect::<Vec<u32>>();

        &self.batches[..] == &counts[..]
    }

    fn possible_solution_count(&self) -> usize {
        let options = self.generate_combinations();
        let count = options
            .par_iter()
            .filter(|option| self.check_option(option))
            .count();
        count
    }
}

fn solution_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(first, second)| {
            let batches = second
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let batches = batches;

            let spaces_to_fill = first.chars().filter(|c| c == &'?').count() as u32;

            Game {
                spaces_to_fill,
                line: first.to_string(),
                batches,
            }
        })
        .map(|puzzle| puzzle.possible_solution_count())
        .sum()
}

fn solution_part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = r#"???.### 1,1,3"#.trim();

        assert_eq!(solution_part_1(input), 1);
    }

    #[test]
    fn part1_2() {
        let input = r#".??..??...?##. 1,1,3"#.trim();

        assert_eq!(solution_part_1(input), 4);
    }

    #[test]
    fn part1_3() {
        let input = r#"?#?#?#?#?#?#?#? 1,3,1,6"#.trim();

        assert_eq!(solution_part_1(input), 1);
    }

    #[test]
    fn part1_4() {
        let input = r#"????.#...#... 4,1,1"#.trim();

        assert_eq!(solution_part_1(input), 1);
    }

    #[test]
    fn part1_5() {
        let input = r#"????.######..#####. 1,6,5"#.trim();

        assert_eq!(solution_part_1(input), 4);
    }

    #[test]
    fn part1_6() {
        let input = r#"?###???????? 3,2,1"#.trim();

        assert_eq!(solution_part_1(input), 10);
    }
}
