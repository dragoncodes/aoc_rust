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

fn solution_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let points_halves = line.split_once(':').unwrap().1.split_once('|').unwrap();

            let first_half = &points_halves
                .0
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let second_half = &points_halves
                .1
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap());

            let card_matches: u32 = second_half.clone().fold(0, |acc, x| {
                if first_half.contains(&x) {
                    acc + 1
                } else {
                    acc
                }
            });

            if card_matches > 0 {
                u32::pow(2, card_matches - 1)
            } else {
                0
            }
        })
        .sum::<u32>()
}

fn solution_part_2(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .fold(
            vec![0; input.lines().count()],
            |mut cards_copies, (idx, line)| {
                let card_and_rest = line.split_once(':').unwrap();

                let points_half = card_and_rest.1.split_once('|').unwrap();

                let first_half = &points_half
                    .0
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                let second_half = &points_half
                    .1
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap());

                second_half.clone().fold(1, |acc, x| {
                    if first_half.contains(&x) {
                        cards_copies[idx + acc] += cards_copies[idx] + 1;

                        acc + 1
                    } else {
                        acc
                    }
                });

                cards_copies[idx] += 1;

                cards_copies
            },
        )
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .trim();

        assert_eq!(solution_part_1(input), 13);
    }

    #[test]
    fn part1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 15205);
    }

    #[test]
    fn part2() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .trim();

        assert_eq!(solution_part_2(input), 30);
    }

    #[test]
    fn part2_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_2(&input), 6189740);
    }
}
