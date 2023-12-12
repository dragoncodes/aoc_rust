use itertools::*;
use std::{collections::HashMap, time::Instant};

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

trait DistanceCalculator {
    fn distance(&self, other: &Self) -> usize;
}

impl DistanceCalculator for (usize, usize) {
    fn distance(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

fn find_distance_between_galaxies(grid: &Vec<Vec<char>>, dark_matter_force: usize) -> usize {
    let mut galaxies = HashMap::<usize, Vec<usize>>::new();

    let mut y_dark_matters = 0;
    for y in 0..grid.len() {
        let mut found_galaxy = false;

        for x in 0..grid[y].len() {
            let c = grid[y][x];

            if c == '#' {
                found_galaxy = true;

                if galaxies.contains_key(&x) {
                    galaxies
                        .get_mut(&x)
                        .unwrap_or(&mut vec![])
                        .push(y + y_dark_matters);
                } else {
                    galaxies.insert(x, vec![y + y_dark_matters]);
                }
            }
        }

        if !found_galaxy {
            y_dark_matters += dark_matter_force;
        }
    }

    // Find dark matter in Xs
    let mut x_dark_matters = Vec::<usize>::new();
    for (x, _) in grid[0].iter().enumerate() {
        let mut found_galaxy = false;

        for y in 0..grid.len() {
            if grid[y][x].eq(&'#') {
                found_galaxy = true;

                break;
            }
        }

        if !found_galaxy {
            x_dark_matters.push(x);
        }
    }

    galaxies
        .iter()
        .flat_map(|x| {
            galaxies
                .get(x.0)
                .unwrap()
                .iter()
                .map(|y| {
                    let dark_matter_offset =
                        x_dark_matters.iter().filter(|dx| *dx < (x.0)).count() * dark_matter_force;

                    (dark_matter_offset + x.0, *y)
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .tuple_combinations()
        .map(|(a, b)| a.distance(&b))
        .sum()
}

fn solution_part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    find_distance_between_galaxies(&grid, 1)
}

fn solution_part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    find_distance_between_galaxies(&grid, 1000000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
            .trim();

        assert_eq!(solution_part_1(input), 374);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_2(&input), 560822911938);
    }
}
