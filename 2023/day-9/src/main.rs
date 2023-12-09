use itertools::*;
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

fn solution_part_1(input: &str) -> i32 {
    let rows = input.lines().map(|line| {
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec()
    });

    fn solve_part_1(vec: &Vec<i32>, current_sum: i32) -> i32 {
        let next_row_values = vec.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();

        if next_row_values.iter().all(|x| x.eq(&0)) {
            current_sum
        } else {
            solve_part_1(
                &next_row_values,
                next_row_values.last().unwrap() + current_sum,
            )
        }
    }

    rows.map(|row| {
        let sum = solve_part_1(&row, *row.last().unwrap());

        sum
    })
    .sum()
}

fn solution_part_2(input: &str) -> i32 {
    let rows = input.lines().map(|line| {
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec()
    });

    fn solve_part_2(vec: &Vec<i32>) -> i32 {
        let next_row_values = vec.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();

        if next_row_values.iter().all(|x| x.eq(&0)) {
            0
        } else {
            next_row_values.first().unwrap() - solve_part_2(&next_row_values)
        }
    }

    rows.map(|row| row.first().unwrap() - solve_part_2(&row))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            .trim();

        assert_eq!(solution_part_1(input), 114);
    }

    #[test]
    fn part1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 1806615041);
    }

    #[test]
    fn part2() {
        let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            .trim();

        assert_eq!(solution_part_2(input), 2);
    }

    #[test]
    fn part2_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_2(&input), 1211);
    }
}
