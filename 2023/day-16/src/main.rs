use itertools::*;
use std::{collections::HashSet, time::Instant};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn print_illuminated_grid(
    grid: &Vec<Vec<char>>,
    illumation: &Vec<Vec<u32>>,
    position: (usize, usize),
) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if (i, j) == position {
                print!("{}", 'X');
                continue;
            }

            if illumation[i][j] != 0 {
                print!("{}", '#');
            } else {
                print!("{}", grid[i][j]);
            }
        }

        println!();
    }
}

trait ToI32 {
    fn to_i32(self) -> (i32, i32);
}

impl ToI32 for (usize, usize) {
    fn to_i32(self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
    }
}

fn move_beam(
    grid: &Vec<Vec<char>>,
    raw_position: (i32, i32),
    direction: Direction,
    illumation: &mut Vec<Vec<u32>>,
    cache: &mut HashSet<(usize, usize, Direction)>,
) {
    if raw_position.0 < 0
        || raw_position.1 < 0
        || raw_position.0 >= grid.len() as i32
        || raw_position.1 >= grid[0].len() as i32
    {
        return;
    }

    let position = (raw_position.0 as usize, raw_position.1 as usize);

    if cache.contains(&(position.0, position.1, direction.clone())) {
        return;
    }

    cache.insert((position.0, position.1, direction.clone()));

    illumation[position.0][position.1] += 1;

    match direction {
        Direction::N => match grid[position.0][position.1] {
            '.' | '|' => {
                move_beam(
                    grid,
                    (raw_position.0 - 1, raw_position.1),
                    Direction::N,
                    illumation,
                    cache,
                );
            }

            '-' => {
                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 - 1),
                    Direction::W,
                    illumation,
                    cache,
                );

                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 + 1),
                    Direction::E,
                    illumation,
                    cache,
                );
            }

            '\\' => {
                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 - 1),
                    Direction::W,
                    illumation,
                    cache,
                );
            }

            '/' => {
                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 + 1),
                    Direction::E,
                    illumation,
                    cache,
                );
            }
            _ => unreachable!(),
        },
        Direction::S => match grid[position.0][position.1] {
            '.' | '|' => {
                move_beam(
                    grid,
                    (raw_position.0 + 1, raw_position.1),
                    Direction::S,
                    illumation,
                    cache,
                );
            }

            '-' => {
                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 - 1),
                    Direction::W,
                    illumation,
                    cache,
                );

                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 + 1),
                    Direction::E,
                    illumation,
                    cache,
                );
            }

            '\\' => {
                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 + 1),
                    Direction::E,
                    illumation,
                    cache,
                );
            }

            '/' => {
                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 - 1),
                    Direction::W,
                    illumation,
                    cache,
                );
            }
            _ => unreachable!(),
        },

        Direction::E => match grid[position.0][position.1] {
            '.' | '-' => {
                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 + 1),
                    Direction::E,
                    illumation,
                    cache,
                );
            }

            '|' => {
                move_beam(
                    grid,
                    (raw_position.0 - 1, raw_position.1),
                    Direction::N,
                    illumation,
                    cache,
                );

                move_beam(
                    grid,
                    (raw_position.0 + 1, raw_position.1),
                    Direction::S,
                    illumation,
                    cache,
                );
            }
            '\\' => {
                move_beam(
                    grid,
                    (raw_position.0 + 1, raw_position.1),
                    Direction::S,
                    illumation,
                    cache,
                );
            }

            '/' => {
                move_beam(
                    grid,
                    (raw_position.0 - 1, raw_position.1),
                    Direction::N,
                    illumation,
                    cache,
                );
            }
            _ => unreachable!(),
        },

        Direction::W => match grid[position.0][position.1] {
            '.' | '-' => {
                move_beam(
                    grid,
                    (raw_position.0, raw_position.1 - 1),
                    Direction::W,
                    illumation,
                    cache,
                );
            }

            '|' => {
                move_beam(
                    grid,
                    (raw_position.0 - 1, raw_position.1),
                    Direction::N,
                    illumation,
                    cache,
                );

                move_beam(
                    grid,
                    (raw_position.0 + 1, raw_position.1),
                    Direction::S,
                    illumation,
                    cache,
                );
            }
            '\\' => {
                move_beam(
                    grid,
                    (raw_position.0 - 1, raw_position.1),
                    Direction::N,
                    illumation,
                    cache,
                );
            }

            '/' => {
                move_beam(
                    grid,
                    (raw_position.0 + 1, raw_position.1),
                    Direction::S,
                    illumation,
                    cache,
                );
            }
            _ => unreachable!(),
        },
    }
}

fn calculate_illumination(grid: &Vec<Vec<char>>, start: (i32, i32), direction: Direction) -> usize {
    let mut cache: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut illumination = vec![vec![0; grid[0].len()]; grid.len()];

    move_beam(grid, start, direction, &mut illumination, &mut cache);

    illumination.iter().flatten().filter(|&&i| i > 0).count()
}

fn solution_part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (0, 0);

    calculate_illumination(&grid, start, Direction::E)
}

fn solution_part_2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut max_illumination = 0;

    for i in 0..grid[0].len() {
        let start = (0 as i32, i as i32);

        let illumination = calculate_illumination(&grid, start, Direction::S);

        if illumination > max_illumination {
            max_illumination = illumination;
        }
    }

    for i in 0..grid[0].len() {
        let start = ((grid.len() - 1) as i32, i as i32);

        let illumination = calculate_illumination(&grid, start, Direction::N);

        if illumination > max_illumination {
            max_illumination = illumination;
        }
    }

    for i in 0..grid.len() {
        let start = (i as i32, 0 as i32);

        let illumination = calculate_illumination(&grid, start, Direction::E);

        if illumination > max_illumination {
            max_illumination = illumination;
        }
    }

    for i in 0..grid.len() {
        let start = (i as i32, (grid[0].len() - 1) as i32);

        let illumination = calculate_illumination(&grid, start, Direction::W);

        if illumination > max_illumination {
            max_illumination = illumination;
        }
    }

    max_illumination
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
            .trim();

        assert_eq!(solution_part_1(input), 46);
    }

    #[test]
    fn part1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 7210);
    }

    #[test]
    fn part2() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
            .trim();

        assert_eq!(solution_part_2(input), 51);
    }

    #[test]
    fn part2_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_2(&input), 7673);
    }
}
