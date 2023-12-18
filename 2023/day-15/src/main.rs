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

fn christmas_hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| {
        let result = ((acc as u32) + (c as u32)) * 17;
        result % 256
    })
}

fn solution_part_1(input: &str) -> usize {
    input
        .split(',')
        .map(|s| s.trim())
        .map(christmas_hash)
        .sum::<u32>() as usize
}

#[derive(Debug)]
enum Operation {
    Remove,
    AddOrReplace(u32),
}

#[derive(Debug)]
struct Instruction {
    lens_label: String,
    box_number: u32,
    operation: Operation,
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug)]
struct Box {
    lenses: Vec<Lens>,
}

fn calculate_focal_power(boxes: &Vec<Box>) -> usize {
    boxes.iter().enumerate().fold(0, |acc, (idx, b)| {
        let box_power = idx + 1;

        let lens_power = b.lenses.iter().enumerate().fold(0, |lens_acc, l| {
            let (lens_idx, lens) = l;

            let lens_power = (lens_idx + 1) * lens.focal_length as usize;

            lens_acc + lens_power
        });

        acc + (lens_power * box_power)
    })
}

fn solution_part_2(input: &str) -> usize {
    let instructions = input
        .split(',')
        .map(|raw_instruction| {
            let instruction = raw_instruction.trim();

            if instruction.chars().last().unwrap_or('.') == '-' {
                let (lens_label, _) = instruction.split_once('-').unwrap();
                let box_number = christmas_hash(&lens_label);

                return Instruction {
                    lens_label: lens_label.to_string(),
                    box_number,
                    operation: Operation::Remove,
                };
            }

            let (lens_label, focal_length) = instruction.split_once('=').unwrap();
            let focal_length = focal_length.parse::<u32>().unwrap();
            let box_number = christmas_hash(&lens_label);

            Instruction {
                lens_label: lens_label.to_string(),
                box_number,
                operation: Operation::AddOrReplace(focal_length),
            }
        })
        .collect_vec();

    let mut boxes = std::iter::repeat_with(|| Box { lenses: Vec::new() })
        .take(256)
        .collect::<Vec<_>>();

    for instruction in instructions {
        match instruction.operation {
            Operation::Remove => {
                boxes
                    .get_mut(instruction.box_number as usize)
                    .unwrap()
                    .lenses
                    .retain(|lens| lens.label != instruction.lens_label);
            }

            Operation::AddOrReplace(lens_focal_length) => {
                let lens = Lens {
                    label: instruction.lens_label,
                    focal_length: lens_focal_length,
                };

                let christmass_box = boxes.get_mut(instruction.box_number as usize).unwrap();

                if let Some(old_lens) = christmass_box
                    .lenses
                    .iter()
                    .position(|l| l.label == lens.label)
                {
                    christmass_box.lenses[old_lens] = lens;
                } else {
                    christmass_box.lenses.push(lens);
                }
            }
        }
    }

    calculate_focal_power(&boxes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#.trim();

        assert_eq!(solution_part_1(input), 1320);
    }

    #[test]
    fn part1_hasher() {
        let input = r#"HASH"#.trim();

        assert_eq!(christmas_hash(input), 52);
    }

    #[test]
    fn part2() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#.trim();

        assert_eq!(solution_part_2(input), 145);
    }
}
