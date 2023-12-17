use itertools::*;
use std::{collections::HashSet, hash::Hasher, time::Instant};

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

fn can_move_east(stone: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    // Edge of map
    if stone.1 >= grid[stone.0].len() - 1 {
        return false;
    }

    let new_col = stone.1 + 1;

    // The grid has obstacles
    if grid[stone.0][new_col] != '.' {
        return false;
    }

    true
}

fn can_move_west(stone: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    // Edge of map
    if stone.1 == 0 {
        return false;
    }

    let new_col = stone.1 - 1;

    // The grid has obstacles
    if grid[stone.0][new_col] != '.' {
        return false;
    }

    true
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

fn can_move_south(stone: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    // Edge of map
    if stone.0 >= grid.len() - 1 {
        return false;
    }

    let new_row = stone.0 + 1;

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

fn roll_stone_westwards(stone: (usize, usize), grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    if !can_move_west(stone, grid) {
        return stone;
    }

    let new_col = stone.1 - 1;

    grid[stone.0][new_col] = 'O';
    grid[stone.0][stone.1] = '.';

    return roll_stone_westwards((stone.0, new_col), grid);
}

fn roll_stone_southwards(stone: (usize, usize), grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    if !can_move_south(stone, grid) {
        return stone;
    }

    let new_row = stone.0 + 1;

    grid[new_row][stone.1] = 'O';
    grid[stone.0][stone.1] = '.';

    return roll_stone_southwards((new_row, stone.1), grid);
}

fn roll_stone_eastwards(stone: (usize, usize), grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    if !can_move_east(stone, grid) {
        return stone;
    }

    let new_col = stone.1 + 1;

    grid[stone.0][new_col] = 'O';
    grid[stone.0][stone.1] = '.';

    return roll_stone_eastwards((stone.0, new_col), grid);
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

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    for i in (0..grid.len()).rev() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                roll_stone_southwards((i, j), grid);
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                roll_stone_westwards((i, j), grid);
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in (0..grid[i].len()).rev() {
            if grid[i][j] == 'O' {
                roll_stone_eastwards((i, j), grid);
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

#[derive(Eq)]
struct GridStateAtPoint {
    point: usize,
    grid: Vec<Vec<char>>,
}

impl std::hash::Hash for GridStateAtPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
    }
}

impl PartialEq for GridStateAtPoint {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

fn roll_all(grid: &mut Vec<Vec<char>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn solution_part_2(input: &str) -> usize {
    // north, then west, then south, then east
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut seen = HashSet::new();

    for i in 0..1_000_000_000 {
        seen.insert(GridStateAtPoint {
            point: i,
            grid: grid.clone(),
        });

        roll_all(&mut grid);

        if let Some(state) = seen.get(&GridStateAtPoint {
            point: 0,
            grid: grid.to_owned(),
        }) {
            let cycle_len = i + 1 - state.point;
            let remaining = 1_000_000_000 - i - 1;
            let remaining = remaining % cycle_len;

            // Remaining is the number of steps we need to take to
            // from where we are at to get to the same position that
            // 1_000_000_000 steps would have taken us.
            for _ in 0..remaining {
                roll_all(&mut grid);
            }

            break;
        }
    }

    get_total_load(&grid)
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

    #[test]
    fn part2() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        assert_eq!(solution_part_2(input), 64);
    }
}
