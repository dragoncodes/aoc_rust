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

fn can_move_north(stone: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    // Edge of map
    if stone.0 == 0 {
        return false;
    }

    let new_row = stone.0 - 1;

    // The grid has obstacles
    if grid[new_row][stone.1] != '.' {
        return false;
    }

    true
}

fn roll_stone_northwards(stone: (usize, usize), grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    if !can_move_north(stone, grid) {
        return stone;
    }

    let new_row = stone.0 - 1;

    grid[new_row][stone.1] = 'O';
    grid[stone.0][stone.1] = '.';

    return roll_stone_northwards((new_row, stone.1), grid);
}

fn get_total_load(grid: &Vec<Vec<char>>) -> usize {
    let mut total_load: usize = 0;

    for row in 0..grid.len() {
        let mut row_load = 0;

        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                row_load += grid.len() - row;
            }
        }

        total_load += row_load;
    }

    total_load
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                roll_stone_northwards((i, j), grid);
            }
        }
    }
}

fn solution_part_1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    tilt_north(&mut grid);

    get_total_load(&grid)
}

fn solution_part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_vertical() {
        let input = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#
        .trim();

        assert_eq!(solution_part_1(input), 136);
    }

    #[test]
    fn part1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 111979);
    }
}
