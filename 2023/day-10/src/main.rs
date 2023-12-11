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

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
enum Pipe {
    Vertical = b'|',
    Horizontal = b'-',
    TopToRight = b'L',
    TopToLeft = b'J',
    BottomToLeft = b'7',
    BottomToRight = b'F',
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Starting,
}

#[derive(Copy, Clone, Debug)]
struct TileWithCoords {
    tile: Tile,
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
    fn to_vec(&self) -> Vec<TileWithCoords> {
        vec![self.top, self.bottom, self.left, self.right]
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn from_idx(idx: usize) -> Direction {
        match idx {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction index {}", idx),
        }
    }
}

trait CheckAdjacent {
    fn check_adjacent(&self, starting: (usize, usize)) -> Adjacent;
}

impl CheckAdjacent for Vec<Vec<Tile>> {
    fn check_adjacent(&self, starting: (usize, usize)) -> Adjacent {
        let mut adjacent = Adjacent {
            top: TileWithCoords {
                tile: Tile::Ground,
                coords: (0, 0),
            },
            bottom: TileWithCoords {
                tile: Tile::Ground,
                coords: (0, 0),
            },
            left: TileWithCoords {
                tile: Tile::Ground,
                coords: (0, 0),
            },
            right: TileWithCoords {
                tile: Tile::Ground,
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

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '|' => Tile::Pipe(Pipe::Vertical),
            '-' => Tile::Pipe(Pipe::Horizontal),
            'L' => Tile::Pipe(Pipe::TopToRight),
            'J' => Tile::Pipe(Pipe::TopToLeft),
            '7' => Tile::Pipe(Pipe::BottomToLeft),
            'F' => Tile::Pipe(Pipe::BottomToRight),
            '.' => Tile::Ground,
            'S' => Tile::Starting,
            _ => panic!("Invalid tile {}", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Pipe(Pipe::Vertical) => '|',
            Tile::Pipe(Pipe::Horizontal) => '-',
            Tile::Pipe(Pipe::TopToRight) => 'L',
            Tile::Pipe(Pipe::TopToLeft) => 'J',
            Tile::Pipe(Pipe::BottomToLeft) => '7',
            Tile::Pipe(Pipe::BottomToRight) => 'F',
            Tile::Ground => '.',
            Tile::Starting => 'S',
        }
    }

    fn can_connect(&self, other: &Tile, coming_from: Direction) -> bool {
        match self {
            Tile::Pipe(Pipe::Vertical) => match other {
                Tile::Pipe(Pipe::Vertical) => {
                    coming_from == Direction::Up || coming_from == Direction::Down
                }
                Tile::Pipe(Pipe::BottomToRight) => coming_from == Direction::Up,
                Tile::Pipe(Pipe::BottomToLeft) => coming_from == Direction::Down,
                Tile::Pipe(Pipe::TopToLeft) => coming_from == Direction::Up,
                Tile::Pipe(Pipe::TopToRight) => coming_from == Direction::Up,
                _ => false,
            },
            Tile::Pipe(Pipe::Horizontal) => match other {
                Tile::Pipe(Pipe::Horizontal) => {
                    coming_from == Direction::Left || coming_from == Direction::Right
                }
                Tile::Pipe(Pipe::TopToLeft) => {
                    coming_from == Direction::Left || coming_from == Direction::Down
                }
                Tile::Pipe(Pipe::TopToRight) => coming_from == Direction::Right,
                Tile::Pipe(Pipe::BottomToLeft) => coming_from == Direction::Left,

                _ => false,
            },
            Tile::Pipe(Pipe::TopToRight) => match other {
                Tile::Pipe(Pipe::Vertical) => coming_from == Direction::Down,
                Tile::Pipe(Pipe::Horizontal) => coming_from == Direction::Left,
                Tile::Pipe(Pipe::BottomToLeft) => coming_from == Direction::Left,
                Tile::Pipe(Pipe::TopToLeft) => coming_from == Direction::Left,

                _ => false,
            },
            Tile::Pipe(Pipe::TopToLeft) => match other {
                Tile::Pipe(Pipe::Vertical) => coming_from == Direction::Down,
                Tile::Pipe(Pipe::Horizontal) => coming_from == Direction::Right,
                Tile::Pipe(Pipe::BottomToLeft) => coming_from == Direction::Down,
                Tile::Pipe(Pipe::BottomToRight) => {
                    coming_from == Direction::Right || coming_from == Direction::Down
                }
                _ => false,
            },
            Tile::Pipe(Pipe::BottomToLeft) => match other {
                Tile::Pipe(Pipe::Vertical) => coming_from == Direction::Up,
                Tile::Pipe(Pipe::Horizontal) => coming_from == Direction::Right,
                Tile::Pipe(Pipe::TopToRight) => {
                    coming_from == Direction::Right || coming_from == Direction::Down
                }
                Tile::Pipe(Pipe::TopToLeft) => coming_from == Direction::Up,
                Tile::Pipe(Pipe::BottomToRight) => coming_from == Direction::Right,
                _ => false,
            },
            Tile::Pipe(Pipe::BottomToRight) => match other {
                Tile::Pipe(Pipe::Vertical) => coming_from == Direction::Down,
                Tile::Pipe(Pipe::Horizontal) => coming_from == Direction::Left,
                Tile::Pipe(Pipe::TopToLeft) => {
                    coming_from == Direction::Left || coming_from == Direction::Up
                }
                Tile::Pipe(Pipe::BottomToLeft) => coming_from == Direction::Left,
                _ => false,
            },
            Tile::Ground => false,
            Tile::Starting => false,
        }
    }
}

fn solution_part_1(input: &str) -> usize {
    let mut starting_point: (usize, usize) = (0, 0);

    let maze: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(idx, c)| {
                    let tile = Tile::from_char(c);

                    if tile == Tile::Starting {
                        starting_point = (row, idx);
                    }

                    tile
                })
                .collect_vec()
        })
        .collect_vec();

    let starting_points = maze
        .check_adjacent(starting_point)
        .to_vec()
        .iter()
        .map(|x| x.coords)
        .collect_vec();

    println!(
        "Staring point is {:?}",
        maze.check_adjacent(starting_point)
            .to_vec()
            .iter()
            .map(|c| c.tile.to_char())
            .collect_vec()
    );

    let mut beaten_path: Vec<Vec<TileWithCoords>> = vec![vec![]; 4];

    (0..4).enumerate().for_each(|(idx, _)| {
        beaten_path[idx].push(TileWithCoords {
            tile: Tile::Starting,
            coords: starting_point,
        });

        beaten_path[idx].push(TileWithCoords {
            tile: Tile::Starting,
            coords: starting_points[idx],
        })
    });

    let mut idx = 0;

    let res = loop {
        let top_coord = if beaten_path[0].len() > 0 {
            beaten_path[0][beaten_path[0].len() - 1].coords
        } else {
            starting_points[0]
        };
        let top_adjacent = maze.check_adjacent(top_coord).to_vec();

        let top = top_adjacent.iter().enumerate().find(|(idx, y)| {
            beaten_path[0]
                .iter()
                .find(|b| b.coords == y.coords)
                .is_none()
                && maze[top_coord.0][top_coord.1]
                    .can_connect(&y.tile, Direction::from_idx(*idx).inverse())
        });

        let bottom_coord = if beaten_path[1].len() > 0 {
            beaten_path[1].last().unwrap().coords
        } else {
            starting_points[1]
        };

        let bottom_adjacent = maze.check_adjacent(bottom_coord).to_vec();
        let bottom = bottom_adjacent.iter().enumerate().find(|(idx, y)| {
            let is_not_already_walked = beaten_path[1]
                .iter()
                .find(|b| b.coords == y.coords)
                .is_none();

            // println!(
            //     "bottom: cur= {:?}, connect_to={:?} idx = {} can_connect ={}, res={}",
            //     maze[bottom_coord.0][bottom_coord.1],
            //     y,
            //     idx,
            //     maze[bottom_coord.0][bottom_coord.1]
            //         .can_connect(&y.tile, Direction::from_idx(*idx).inverse()),
            //     is_not_already_walked
            //         && maze[bottom_coord.0][bottom_coord.1]
            //             .can_connect(&y.tile, Direction::from_idx(*idx).inverse())
            // );

            is_not_already_walked
                && maze[bottom_coord.0][bottom_coord.1]
                    .can_connect(&y.tile, Direction::from_idx(*idx).inverse())
        });

        let left_coord = if beaten_path[2].len() > 0 {
            beaten_path[2][beaten_path[2].len() - 1].coords
        } else {
            starting_points[2]
        };
        let left_adjacent = maze.check_adjacent(left_coord).to_vec();
        let left = left_adjacent.iter().enumerate().find(|(idx, y)| {
            beaten_path[2]
                .iter()
                .find(|b| b.coords == y.coords)
                .is_none()
                && maze[left_coord.0][left_coord.1]
                    .can_connect(&y.tile, Direction::from_idx(*idx).inverse())
        });

        let right_coord = if beaten_path[3].len() > 0 {
            beaten_path[3][beaten_path[3].len() - 1].coords
        } else {
            starting_points[3]
        };
        let right_adjacent = maze.check_adjacent(right_coord).to_vec();
        let right = right_adjacent.iter().enumerate().find(|(idx, y)| {
            beaten_path[3]
                .iter()
                .find(|b| b.coords == y.coords)
                .is_none()
                && maze[right_coord.0][right_coord.1]
                    .can_connect(&y.tile, Direction::from_idx(*idx).inverse())
        });

        if let Some((_, top)) = top {
            println!(
                "Connecting {:?} to {:?}",
                maze[top_coord.0][top_coord.1], top
            );
            beaten_path[0].push(TileWithCoords {
                tile: top.tile,
                coords: top.coords,
            });
        }

        if let Some((_, bottom)) = bottom {
            beaten_path[1].push(TileWithCoords {
                tile: bottom.tile,
                coords: bottom.coords,
            });
        }

        if let Some((_, left)) = left {
            beaten_path[2].push(TileWithCoords {
                tile: left.tile,
                coords: left.coords,
            });
        }

        if let Some((_, right)) = right {
            // println!(
            //     "Connecting {:?} to {:?}",
            //     maze[right_coord.0][right_coord.1], right
            // );

            beaten_path[3].push(TileWithCoords {
                tile: right.tile,
                coords: right.coords,
            });
        }

        idx += 1;

        println!(
            "top Beaten path {:?}",
            beaten_path[0]
                .iter()
                .map(|x| (x.coords, x.tile.to_char()))
                .last()
        );
        //
        // println!(
        //     "bottom Beaten path {:?}",
        //     beaten_path[3].iter().map(|x| x.coords).last()
        // );

        // println!(
        //     "FINAL: {:?}",
        //     beaten_path
        //         .iter()
        //         .filter(|x| x.len() > 0)
        //         .map(|x| x.last().unwrap().coords)
        //         .collect_vec()
        // );

        if beaten_path
            .iter()
            .filter(|x| x.len() > 0)
            .map(|x| x.last().unwrap().coords)
            .tuple_combinations()
            .any(|(a, b)| {
                // println!("a = {:?}, b = {:?}", a, b);
                a == b
            })
        {
            println!("Found a loop");
            break idx + 1;
        }
    };

    println!(
        "Beaten path {:?}",
        beaten_path[3]
            .iter()
            .map(|x| x.tile)
            .map(|t| t.to_char())
            .collect_vec()
    );

    res
}

fn solution_part_2(input: &str) -> u32 {
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
}
