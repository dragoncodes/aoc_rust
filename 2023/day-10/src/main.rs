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

#[derive(Copy, Clone, Debug)]
struct TileWithCoords {
    tile: char,
    coords: (usize, usize),
}

#[derive(Debug)]
struct Adjacent {
    top: TileWithCoords,
    bottom: TileWithCoords,
    left: TileWithCoords,
    right: TileWithCoords,
}

impl Adjacent {
    fn into_map(&self) -> HashMap<Direction, TileWithCoords> {
        let mut map = HashMap::new();

        map.insert(Direction::N, self.top);
        map.insert(Direction::S, self.bottom);
        map.insert(Direction::W, self.left);
        map.insert(Direction::E, self.right);

        map
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
        }
    }

    fn from_idx(idx: usize) -> Direction {
        match idx {
            0 => Direction::N,
            1 => Direction::S,
            2 => Direction::W,
            3 => Direction::E,
            _ => panic!("Invalid direction index {}", idx),
        }
    }
}

trait CanConnectChecker {
    fn can_connect(&self, other: &Self, coming_from: Direction) -> bool;
}

impl CanConnectChecker for char {
    fn can_connect(&self, other: &Self, coming_from: Direction) -> bool {
        match (self, other) {
            ('|', '|') => coming_from == Direction::N || coming_from == Direction::S,
            ('|', 'L') => coming_from == Direction::N,
            ('|', 'J') => coming_from == Direction::N,
            ('|', 'F') => coming_from == Direction::S,
            ('|', '7') => coming_from == Direction::S,
            ('|', _) => false,

            ('-', '-') => coming_from == Direction::E || coming_from == Direction::W,
            ('-', '7') => coming_from == Direction::W,
            ('-', 'J') => coming_from == Direction::W,
            ('-', 'F') => coming_from == Direction::E,
            ('-', 'L') => coming_from == Direction::E,
            ('-', _) => false,

            ('L', '|') => coming_from == Direction::S,
            ('L', '-') => coming_from == Direction::W,
            ('L', 'J') => coming_from == Direction::W,
            ('L', '7') => coming_from == Direction::S || coming_from == Direction::W,
            ('L', 'F') => coming_from == Direction::S,
            ('L', _) => false,

            ('J', '|') => coming_from == Direction::S,
            ('J', '-') => coming_from == Direction::E,
            ('J', 'L') => coming_from == Direction::E,
            ('J', '7') => coming_from == Direction::S,
            ('J', 'F') => coming_from == Direction::S || coming_from == Direction::E,
            ('J', _) => false,

            ('7', '|') => coming_from == Direction::N,
            ('7', '-') => coming_from == Direction::E,
            ('7', 'L') => coming_from == Direction::N || coming_from == Direction::E,
            ('7', 'J') => coming_from == Direction::N,
            ('7', 'F') => coming_from == Direction::E,
            ('7', _) => false,

            ('F', '|') => coming_from == Direction::N,
            ('F', '-') => coming_from == Direction::W,
            ('F', 'L') => coming_from == Direction::N,
            ('F', 'J') => coming_from == Direction::N || coming_from == Direction::W,
            ('F', '7') => coming_from == Direction::W,
            ('F', _) => false,

            ('S', '|') => coming_from == Direction::N || coming_from == Direction::S,
            ('S', '-') => coming_from == Direction::W || coming_from == Direction::E,
            ('S', 'L') => coming_from == Direction::N || coming_from == Direction::E,
            ('S', 'J') => coming_from == Direction::N || coming_from == Direction::W,
            ('S', '7') => coming_from == Direction::W || coming_from == Direction::S,
            ('S', 'F') => coming_from == Direction::W || coming_from == Direction::E,

            ('.', '.') => false,
            ('.', _) => false,
            (_, '.') => false,
            _ => panic!("Unexpected tile types {} {}", self, other),
        }
    }
}

trait CheckAdjacent {
    fn check_adjacent(&self, starting: (usize, usize)) -> Adjacent;
}

impl CheckAdjacent for Vec<Vec<char>> {
    fn check_adjacent(&self, starting: (usize, usize)) -> Adjacent {
        let mut adjacent = Adjacent {
            top: TileWithCoords {
                tile: '.',
                coords: (0, 0),
            },
            bottom: TileWithCoords {
                tile: '.',
                coords: (0, 0),
            },
            left: TileWithCoords {
                tile: '.',
                coords: (0, 0),
            },
            right: TileWithCoords {
                tile: '.',
                coords: (0, 0),
            },
        };

        if starting.0 > 0 {
            adjacent.top = TileWithCoords {
                tile: self[starting.0 - 1][starting.1],
                coords: (starting.0 - 1, starting.1),
            };
        }

        if starting.0 < self.len() - 1 {
            adjacent.bottom = TileWithCoords {
                tile: self[starting.0 + 1][starting.1],
                coords: (starting.0 + 1, starting.1),
            };
        }

        if starting.1 > 0 {
            adjacent.left = TileWithCoords {
                tile: self[starting.0][starting.1 - 1],
                coords: (starting.0, starting.1 - 1),
            };
        }

        if starting.1 < self[starting.0].len() - 1 {
            adjacent.right = TileWithCoords {
                tile: self[starting.0][starting.1 + 1],
                coords: (starting.0, starting.1 + 1),
            };
        }

        adjacent
    }
}

type BeatenPath = HashMap<Direction, Vec<TileWithCoords>>;

fn solution_part_1(input: &str) -> usize {
    let mut starting_point: (usize, usize) = (0, 0);

    let maze: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(idx, c)| {
                    if c.eq(&'S') {
                        starting_point = (row, idx);
                    }

                    c
                })
                .collect_vec()
        })
        .collect_vec();

    let mut beaten_path: BeatenPath = HashMap::new();

    {
        let starting_tile = TileWithCoords {
            tile: 'S',
            coords: starting_point,
        };

        beaten_path.insert(Direction::N, vec![starting_tile]);
        beaten_path.insert(Direction::S, vec![starting_tile]);
        beaten_path.insert(Direction::W, vec![starting_tile]);
        beaten_path.insert(Direction::E, vec![starting_tile]);
    }

    let mut idx = 0;

    fn get_next_tile_for_dir(
        maze: &Vec<Vec<char>>,
        beaten_path: &BeatenPath,
        direction: Direction,
    ) -> Option<TileWithCoords> {
        let coord = beaten_path[&direction][beaten_path[&direction].len() - 1].coords;
        let adjacent = maze.check_adjacent(coord).into_map();

        if beaten_path[&direction].len() == 1 {
            let last = beaten_path[&direction][beaten_path[&direction].len() - 1];

            if !maze[last.coords.0][last.coords.1]
                .can_connect(&adjacent[&direction].tile, direction.inverse())
            {
                return None;
            }

            return Some(adjacent[&direction]);
        }

        let next = adjacent.iter().find(|entry| {
            beaten_path[&direction]
                .iter()
                .find(|b| b.coords == entry.1.coords)
                .is_none()
                && maze[coord.0][coord.1].can_connect(&entry.1.tile, entry.0.inverse())
        });

        next.map(|x| x.1.clone())
    }

    loop {
        let top = get_next_tile_for_dir(&maze, &beaten_path, Direction::N);
        let bottom = get_next_tile_for_dir(&maze, &beaten_path, Direction::S);
        let left = get_next_tile_for_dir(&maze, &beaten_path, Direction::W);
        let right = get_next_tile_for_dir(&maze, &beaten_path, Direction::E);

        if let Some(top) = top {
            beaten_path
                .get_mut(&Direction::N)
                .unwrap()
                .push(TileWithCoords {
                    tile: top.tile,
                    coords: top.coords,
                });
        }

        if let Some(bottom) = bottom {
            beaten_path
                .get_mut(&Direction::S)
                .unwrap()
                .push(TileWithCoords {
                    tile: bottom.tile,
                    coords: bottom.coords,
                });
        }

        if let Some(left) = left {
            beaten_path
                .get_mut(&Direction::W)
                .unwrap()
                .push(TileWithCoords {
                    tile: left.tile,
                    coords: left.coords,
                });
        }

        if let Some(right) = right {
            beaten_path
                .get_mut(&Direction::E)
                .unwrap()
                .push(TileWithCoords {
                    tile: right.tile,
                    coords: right.coords,
                });
        }

        idx += 1;

        if beaten_path
            .iter()
            .filter(|x| x.1.len() > 1)
            .map(|x| x.1.last().unwrap().coords)
            .tuple_combinations()
            .any(|(a, b)| a == b)
        {
            break idx;
        }
    }
}

fn solution_part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#
            .trim();

        assert_eq!(solution_part_1(input), 8);
    }

    #[test]
    fn part1_1() {
        let input = r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#
            .trim();

        assert_eq!(solution_part_1(input), 4);
    }

    #[test]
    fn part_1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 6613);
    }
}
