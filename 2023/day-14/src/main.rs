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

fn can_move_north(
    stone: (usize, usize),
    grid: &Vec<Vec<char>>,
    round_stones: &mut HashMap<(usize, usize), bool>,
) -> bool {
    // Edge of map
    if stone.0 == 0 {
        return false;
    }

    let new_row = stone.0 - 1;

    // The grid has obstacles
    if grid[new_row][stone.1] == '#' {
        return false;
    }

    // There is a round stone in the way
    if let Some(_) = round_stones.get(&(new_row, stone.1)) {
        return false;
    }

    true
}

fn roll_stone_northwards(
    stone: (usize, usize),
    grid: &Vec<Vec<char>>,
    round_stones: &mut HashMap<(usize, usize), bool>,
) -> (usize, usize) {
    if !can_move_north(stone, grid, round_stones) {
        return stone;
    }

    let new_row = stone.0 - 1;

    round_stones.insert((new_row, stone.1), true);
    round_stones.remove(&stone);

    return roll_stone_northwards((new_row, stone.1), grid, round_stones);
}

fn get_total_load(grid: &Vec<Vec<char>>, round_stones: &HashMap<(usize, usize), bool>) -> usize {
    let mut total_load: usize = 0;

    for row in 0..grid.len() {
        let mut row_load = 0;

        for col in 0..grid[row].len() {
            if let Some(stone) = round_stones.get(&(row, col)) {
                if *stone {
                    row_load += grid.len() - row;
                }
            }
        }

        total_load += row_load;
    }

    total_load
}

fn tilt_north(grid: &Vec<Vec<char>>, round_stones: &mut HashMap<(usize, usize), bool>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                roll_stone_northwards((i, j), grid, round_stones);
            }
        }
    }
}

fn solution_part_1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut round_stones: HashMap<(usize, usize), bool> = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                round_stones.insert((i, j), true);
            }
        }
    }

    tilt_north(&grid, &mut round_stones);

    get_total_load(&grid, &round_stones)
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
}
