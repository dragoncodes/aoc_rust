use card::*;
use std::{cmp::Ordering, time::Instant};

pub mod card;

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

fn solution_part_1(input: &str) -> usize {
    let mut games = input
        .lines()
        .map(|line| {
            let cards = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .chars()
                .map(|x| Card::from_char(x, false))
                .collect::<Vec<Card>>();

            let bid = line.split_whitespace().nth(1).unwrap().parse().unwrap();

            Game {
                hand: Hand {
                    cards: cards.clone(),
                    hand_type: cards.determine_hand_type(),
                },
                bid,
            }
        })
        .collect::<Vec<Game>>();

    games.sort();

    games
        .iter()
        .enumerate()
        .map(|(idx, game)| (idx + 1) * game.bid)
        .sum()
}

fn solution_part_2(input: &str) -> usize {
    let mut games = input
        .lines()
        .map(|line| {
            let cards = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .chars()
                .map(|x| Card::from_char(x, true))
                .collect::<Vec<Card>>();

            let bid = line.split_whitespace().nth(1).unwrap().parse().unwrap();

            Game {
                hand: Hand {
                    cards: cards.clone(),
                    hand_type: cards.determine_hand_type(),
                },
                bid,
            }
        })
        .collect::<Vec<Game>>();

    games.sort();

    games
        .iter()
        .enumerate()
        .map(|(idx, game)| (idx + 1) * game.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#
            .trim();

        assert_eq!(solution_part_1(input), 6440);
    }

    #[test]
    fn part2() {
        let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#
            .trim();

        assert_eq!(solution_part_2(input), 5905);
    }

    #[test]
    fn part2_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_2(&input), 254083736);
    }
}
