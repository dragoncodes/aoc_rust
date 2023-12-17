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

trait ReflectionFinder {
    fn find_symmetry(&mut self) -> Option<Symmetry>;
}

fn find_vertical_symmertry(counts: &Vec<u32>) -> Option<usize> {
    let mut col = 0;
    while col < counts.len() - 1 {
        let mirror_half_size = (col + 1).min(counts.len() - col - 1);

        let mut reflected = true;
        for offset in 0..mirror_half_size {
            if counts[col - offset] != counts[col + offset + 1] {
                reflected = false;
                break;
            }
        }

        if reflected {
            return Some(col);
        }

        col += 1;
    }

    None
}

fn find_horizontal_symmetry(counts: &Vec<u32>) -> Option<usize> {
    let mut row = 0;
    while row < counts.len() - 1 {
        let mirror_half_size = (row + 1).min(counts.len() - row - 1);

        let mut reflected = true;
        for offset in 0..mirror_half_size {
            if counts[row - offset] != counts[row + offset + 1] {
                reflected = false;
                break;
            }
        }

        if reflected {
            return Some(row);
        }

        row += 1;
    }

    None
}

enum SymmetryType {
    Vertical,
    Horizontal,
}

struct Symmetry {
    length: usize,
    symmetry_type: SymmetryType,
}

impl ReflectionFinder for Vec<Vec<char>> {
    fn find_symmetry(&mut self) -> Option<Symmetry> {
        let mut rows: Vec<u32> = vec![];
        let mut cols: HashMap<usize, Vec<_>> = HashMap::new();

        for i in 0..self.len() {
            let mut row_butwise = vec![];

            for j in 0..self[0].len() {
                let c = match self[i][j] {
                    '#' => '1',
                    '.' => '0',
                    _ => panic!("Oh no, invalid character: {}", self[i][j]),
                };

                row_butwise.push(c);
                cols.entry(j)
                    .and_modify(|char_col| char_col.push(c))
                    .or_insert(vec![c]);
            }

            rows.push(u32::from_str_radix(&row_butwise.iter().join(""), 2).unwrap());
        }

        let cols: Vec<u32> = (0..(*cols.keys().max().unwrap() + 1))
            .into_iter()
            .map(|col| u32::from_str_radix(&cols[&col].iter().join(""), 2).unwrap())
            .collect_vec();

        let vertical_symmetry = find_vertical_symmertry(&cols);
        if let Some(vertical_symmetry) = vertical_symmetry {
            return Some(Symmetry {
                length: vertical_symmetry,
                symmetry_type: SymmetryType::Vertical,
            });
        }

        let horizonta_summetry = find_horizontal_symmetry(&rows);
        if let Some(horizonta_summetry) = horizonta_summetry {
            return Some(Symmetry {
                length: horizonta_summetry,
                symmetry_type: SymmetryType::Horizontal,
            });
        }

        None
    }
}

fn solution_part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|puzzle| {
            puzzle
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .map(|mut puzzle| puzzle.find_symmetry())
        .map(|symmetry| match symmetry {
            Some(symmetry) => match symmetry.symmetry_type {
                SymmetryType::Vertical => symmetry.length + 1,
                SymmetryType::Horizontal => (symmetry.length + 1) * 100,
            },

            None => 0,
        })
        .sum::<usize>()
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
#.##..##.
..#.##.#.
##......# ##......#
..#.##.#.
..##..##.
#.#.##.#."#
            .trim();

        assert_eq!(solution_part_1(input), 5);
    }

    #[test]
    fn part1_horizontal() {
        let input = r#"
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
            .trim();

        assert_eq!(solution_part_1(input), 400);
    }

    #[test]
    fn part1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 32035);
    }
}
