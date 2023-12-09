use itertools::{FoldWhile, Itertools};
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

#[repr(u8)]
#[derive(Debug)]
enum Instruction {
    Right = b'R',
    Left = b'L',
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            b'R' => Instruction::Right,
            b'L' => Instruction::Left,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug)]
struct Branches {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Node {
    name: String,
    branches: Branches,
}

fn solution_part_1(input: &str) -> String {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let instructions = instructions
        .chars()
        .map(|c| Instruction::from(c as u8))
        .collect_vec();

    let nodes = nodes
        .lines()
        .map(|line| {
            let (name, branches) = line.split_once(" = ").unwrap();
            let (left, right) = branches.split_once(", ").unwrap();

            Node {
                name: name.to_string(),
                branches: Branches {
                    left: left.replace("(", ""),
                    right: right.replace(")", ""),
                },
            }
        })
        .fold(HashMap::new(), |mut acc, node| {
            acc.insert(node.name.clone(), node);
            acc
        });

    instructions
        .iter()
        .cycle()
        .enumerate()
        .fold_while(String::from("AAA"), |acc, (idx, instruction)| {
            let next_node = match instruction {
                Instruction::Right => &nodes.get(&acc).unwrap().branches.right,
                Instruction::Left => &nodes.get(&acc).unwrap().branches.left,
            };

            if next_node == "ZZZ" {
                FoldWhile::Done((idx + 1).to_string())
            } else {
                FoldWhile::Continue(next_node.to_string())
            }
        })
        .into_inner()
}

fn solution_part_2(input: &str) -> usize {
    // NOTE TO READER:
    // The solution idea is not original copied from a good guy on reddit
    // The general purpose solution does not work for the size of the input, unless you've got time
    // (who does these days??)

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b > 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }

        a
    }

    fn lcm(nums: &[usize]) -> usize {
        if nums.len() == 1 {
            return nums[0];
        }
        let a = nums[0];
        let b = lcm(&nums[1..]);
        a * b / gcd(a, b)
    }

    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let instructions = instructions
        .chars()
        .map(|c| Instruction::from(c as u8))
        .collect_vec();

    let nodes = nodes
        .lines()
        .map(|line| {
            let (name, branches) = line.split_once(" = ").unwrap();
            let (left, right) = branches.split_once(", ").unwrap();

            let left = left.replace("(", "");
            let right = right.replace(")", "");

            Node {
                name: name.to_string(),
                branches: Branches { left, right },
            }
        })
        .fold(HashMap::new(), |mut acc, node| {
            acc.insert(node.name.clone(), node);
            acc
        });

    let starting_nodes = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|node_name| nodes.get(node_name).unwrap());

    let cycles = starting_nodes
        .map(|node| {
            let mut current_node = node;

            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(idx, instruction)| {
                    current_node = match instruction {
                        Instruction::Right => nodes.get(&current_node.branches.right).unwrap(),

                        Instruction::Left => nodes.get(&current_node.branches.left).unwrap(),
                    };

                    if current_node.name.ends_with("Z") {
                        Some(idx + 1)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .collect_vec();

    lcm(&cycles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#
            .trim();

        assert_eq!(solution_part_1(input), "2");
    }

    #[test]
    fn part1_1() {
        let input = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            .trim();

        assert_eq!(solution_part_1(input), "6");
    }

    #[test]
    fn part_2() {
        let input = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            .trim();

        assert_eq!(solution_part_2(&input), 6);
    }
}
