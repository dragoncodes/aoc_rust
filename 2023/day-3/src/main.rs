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

fn parse_str_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

trait SpecialCharactersChecker {
    fn is_engine_part(&self) -> bool;
    fn is_gear_part(&self) -> bool;
}

impl SpecialCharactersChecker for char {
    fn is_engine_part(&self) -> bool {
        if self.is_digit(10) {
            return false;
        }

        match self {
            '.' => false,
            _ => true,
        }
    }

    fn is_gear_part(&self) -> bool {
        self.eq(&'*')
    }
}

struct Vec2 {
    x: usize,
    y: usize,
}

struct Range {
    start: usize,
    end: usize,
}

trait GridChecker<F, T>
where
    F: Fn(T) -> bool,
{
    fn check_grid_around(&self, x: usize, range: Range, checker: F) -> Option<Vec2>;
}

impl<F> GridChecker<F, char> for Vec<Vec<char>>
where
    F: Fn(char) -> bool,
{
    fn check_grid_around(&self, x: usize, range: Range, checker: F) -> Option<Vec2> {
        let y = range.start;
        let range_end = range.end;

        // Top side
        if x > 0 {
            // Top left diagonal
            if y > 0 {
                if checker(self[x - 1][y - 1]) {
                    return Some(Vec2 { x: x - 1, y: y - 1 });
                }
            }

            // Top right diagonal
            if y + range_end - 1 < self[x].len() - 1 {
                if checker(self[x - 1][y + 1 + range_end - 1]) {
                    return Some(Vec2 {
                        x: x - 1,
                        y: y + 1 + range_end - 1,
                    });
                }
            }

            for i in y..(y + range_end) {
                if checker(self[x - 1][i]) {
                    return Some(Vec2 { x: x - 1, y: i });
                }
            }
        }

        // Bottom Side
        if x < self.len() - 1 {
            // Bottom left diagonal
            if y > 0 {
                if checker(self[x + 1][y - 1]) {
                    return Some(Vec2 { x: x + 1, y: y - 1 });
                }
            }

            // Bottom right diagonal
            if y + range_end - 1 < self[x].len() - 1 {
                if checker(self[x + 1][y + 1 + range_end - 1]) {
                    return Some(Vec2 {
                        x: x + 1,
                        y: y + 1 + range_end - 1,
                    });
                }
            }

            // Directly bottom
            for i in y..(y + range_end) {
                if checker(self[x + 1][i]) {
                    return Some(Vec2 { x: x + 1, y: i });
                }
            }
        }

        // Left
        if y > 0 {
            if checker(self[x][y - 1]) {
                return Some(Vec2 { x, y: y - 1 });
            }
        }

        // Right
        if y + range_end - 1 < self[x].len() - 1 && checker(self[x][y + 1 + range_end - 1]) {
            return Some(Vec2 {
                x,
                y: y + 1 + range_end - 1,
            });
        }

        None
    }
}

fn solution_part_1(input: &str) -> i32 {
    let matrix = parse_str_to_matrix(input);

    let mut buff = String::new();
    let mut answer: i32 = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j].is_digit(10) {
                buff.push(matrix[i][j]);
            }

            if j + 1 < matrix[i].len() && !matrix[i][j + 1].is_digit(10) {
                if buff.is_empty() {
                    continue;
                }

                let buff_len = buff.len();

                let num = buff.parse::<i32>().unwrap();

                buff.clear();

                if let Some(_) = matrix.check_grid_around(
                    i,
                    Range {
                        start: j - (buff_len - 1),
                        end: buff_len,
                    },
                    |c| c.is_engine_part(),
                ) {
                    answer += num;

                    continue;
                }
            }
        }

        if buff.len() > 0 {
            let num = buff.parse::<i32>().unwrap();

            if let Some(_) = matrix.check_grid_around(
                i,
                Range {
                    start: (matrix[i].len() - 1) - (buff.len() - 1),
                    end: buff.len(),
                },
                |c| c.is_engine_part(),
            ) {
                answer += num;
            }
        }

        buff.clear();
    }

    answer
}

fn solution_part_2(input: &str) -> i32 {
    let matrix = parse_str_to_matrix(input);

    let mut touches: HashMap<String, Vec<i32>> = HashMap::new();

    let mut buff = String::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j].is_digit(10) {
                buff.push(matrix[i][j]);
            }

            if j + 1 < matrix[i].len() && !matrix[i][j + 1].is_digit(10) {
                if buff.is_empty() {
                    continue;
                }

                let buff_len = buff.len();

                let num = buff.parse::<i32>().unwrap();

                buff.clear();

                if let Some(touching_gear) = matrix.check_grid_around(
                    i,
                    Range {
                        start: j - (buff_len - 1),
                        end: buff_len,
                    },
                    |c| c.is_gear_part(),
                ) {
                    let key = format!("{}-{}", touching_gear.x, touching_gear.y);

                    if touches.get(&key).is_none() {
                        touches.insert(key, vec![num]);
                    } else {
                        touches.get_mut(&key).unwrap().push(num);
                    }

                    continue;
                }
            }
        }

        if buff.len() > 0 {
            let num = buff.parse::<i32>().unwrap();

            if let Some(touching_gear) = matrix.check_grid_around(
                i,
                Range {
                    start: (matrix[i].len() - 1) - (buff.len() - 1),
                    end: buff.len(),
                },
                |c| c.is_gear_part(),
            ) {
                let key = format!("{}-{}", touching_gear.x, touching_gear.y);

                if touches.get(&key).is_none() {
                    touches.insert(key, vec![num]);
                } else {
                    touches.get_mut(&key).unwrap().push(num);
                }
            }
        }

        buff.clear();
    }

    touches
        .values()
        .map(|v| {
            if v.len() == 2 {
                v.into_iter().fold(1, |acc, v| acc * v)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .trim();

        assert_eq!(solution_part_1(input), 4361);
    }

    #[test]
    fn part1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 512794);
    }

    #[test]
    fn part2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .trim();

        assert_eq!(solution_part_2(input), 467835);
    }

    #[test]
    fn part2_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_2(&input), 67779080);
    }
}
