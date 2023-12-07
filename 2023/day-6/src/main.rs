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

fn count_winning_combinations(time: usize, distance_to_beat: usize) -> usize {
    (1..=time)
        .find_map(|idx| {
            let speed_left = idx;

            let race_time_left = time - idx;

            let distance_left = race_time_left * speed_left;

            if distance_left > distance_to_beat {
                Some((idx..=(time - idx)).count())
            } else {
                None
            }
        })
        .unwrap()
}

fn solution_part_1(input: &str) -> usize {
    let vecs = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let times = vecs.get(0).unwrap();
    let millimeters = vecs.get(1).unwrap();

    (0..times.len())
        .map(|round| {
            let time = times.get(round).unwrap();
            let distance_to_beat = millimeters.get(round).unwrap();

            count_winning_combinations(*time, *distance_to_beat)
        })
        .fold(1, |acc, x| acc * x)
}

fn solution_part_2(input: &str) -> usize {
    let vecs = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|line| {
            line.split_whitespace()
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(&x);

                    acc
                })
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>();

    let time = vecs.get(0).unwrap();
    let distance_to_beat = vecs.get(1).unwrap();

    count_winning_combinations(*time, *distance_to_beat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
Time:      7  15   30
Distance:  9  40  200"#
            .trim();

        assert_eq!(solution_part_1(input), 288);
    }

    #[test]
    fn part2() {
        let input = r#"
Time:      7  15   30
Distance:  9  40  200"#
            .trim();

        assert_eq!(solution_part_2(input), 71503);
    }
}
