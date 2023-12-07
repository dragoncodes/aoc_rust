use std::{cmp::Ordering, time::Instant};

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

#[derive(Debug, PartialOrd, Ord, Clone)]
#[repr(u8)]
enum Card {
    Number(u8),
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 15,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Card::Number(a), Card::Number(b)) => a == b,
            (Card::Number(_), _) => false,
            (_, Card::Number(_)) => false,
            (Card::Ten, Card::Ten) => true,
            (Card::Jack, Card::Jack) => true,
            (Card::Queen, Card::Queen) => true,
            (Card::King, Card::King) => true,
            (Card::Ace, Card::Ace) => true,
            _ => false,
        }
    }
}

impl Eq for Card {
    fn assert_receiver_is_total_eq(&self) {
        // This is a no-op because `Card` is total-eq.
    }
}

impl Card {
    fn from_char(c: char, use_joker: bool) -> Card {
        match c {
            'J' => {
                if use_joker {
                    Card::Number(0)
                } else {
                    Card::Jack
                }
            }
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            'T' => Card::Ten,
            _ => Card::Number(c.to_digit(10).unwrap().try_into().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let are_hand_types_same = self.hand_type == other.hand_type;

        if are_hand_types_same {
            for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if card != other_card {
                    return Some(card.cmp(other_card));
                }
            }

            return None;
        } else {
            return self.hand_type.partial_cmp(&other.hand_type);
        }
    }
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: usize,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Game {
    fn assert_receiver_is_total_eq(&self) {
        // This is a no-op because `Game` is total-eq.
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

trait CardEq {
    fn eq_card(&self, other: &Self, use_joker: bool) -> bool;
}

trait HandTypeDeterminer {
    fn determine_hand_type(&self) -> HandType;
}

impl HandTypeDeterminer for Vec<Card> {
    fn determine_hand_type(&self) -> HandType {
        let mut cards = self.to_vec();

        cards.sort();

        let is_five_of_kind = {
            cards[0] == cards[4] || (
                    // Jokers
                    // JJJJ<other>
                    cards[3] == Card::Number(0) 

                    // JJJ<other><other>
                    || (cards[2] == Card::Number(0) && cards[3] == cards[4] )

                    // JJ<other><other><other>
                    || (cards[1] == Card::Number(0) && cards[2] == cards[4] )

                    // J<other><other><other><other>
                    || (cards[0] == Card::Number(0) && cards[1] == cards[4] ))
        };

        if is_five_of_kind {
            return HandType::FiveOfKind;
        }

        let is_four_of_kind = {
            cards[0] == cards[3]
                || cards[1] == cards[4]
                || (
                    // Jokers
                    // JJJ<other><other>
                    cards[2] == Card::Number(0)

                    // JJ<other><other><other>
                    || (cards[1] == Card::Number(0) && (cards[2] == cards[3] || cards[3] == cards[4] ) )

                    // J<other><other><other><other>
                    || (cards[0] == Card::Number(0) && (cards[1] == cards[3] || cards[2] == cards[4]))
                )
        };

        if is_four_of_kind {
            return HandType::FourOfKind;
        }

        let is_full_house = {
            let first = &cards[0];
            let second = &cards[1];
            let third = &cards[2];
            let fourth = &cards[3];
            let fifth = &cards[4];

            (first == second && second == third && fourth == fifth)
                || (first == second && third == fourth && fourth == fifth)

            // Jokers
            // JJ<other><other><other>
            || (*second == Card::Number(0) && (fourth == fifth))

            // J<other><other><other><other>
            || (*first == Card::Number(0) &&  ((second == third && fourth == fifth) ))
        };

        if is_full_house {
            return HandType::FullHouse;
        }

        let is_three_of_kind = {
            let first = &cards[0];
            let second = &cards[1];
            let third = &cards[2];
            let fourth = &cards[3];
            let fifth = &cards[4];

            (first == second && second == third)
                || (second == third && third == fourth)
                || (third == fourth && fourth == fifth)

            // Jokers
            // JJ<other><other><other>
            ||  *second == Card::Number(0) 

            // J<other><other><other><other>
            || (*first == Card::Number(0) && (second == third || third == fourth || fourth == fifth ))
        };

        if is_three_of_kind {
            return HandType::ThreeOfKind;
        }

        let is_two_pair = {
            let first = &cards[0];
            let second = &cards[1];
            let third = &cards[2];
            let fourth = &cards[3];
            let fifth = &cards[4];

            (first == second && third == fourth)
                || (second == third && fourth == fifth)
                || (first == second && fourth == fifth)
            // Jokers
            || 
            // J<other><other><other><other>
            (*first == Card::Number(0) && (second == third || second == fourth || second == fifth || third == fourth || third == fifth || fourth == fifth))
        };

        if is_two_pair {
            return HandType::TwoPair;
        }

        let is_one_pair = {
            let first = &cards[0];
            let second = &cards[1];
            let third = &cards[2];
            let fourth = &cards[3];
            let fifth = &cards[4];

            first == second || second == third || third == fourth || fourth == fifth 
                // Jokers
                // J<other><other><other><other>
                || (*first == Card::Number(0))
        };

        if is_one_pair {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
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
        .map(|(idx, game)| {
            (idx + 1) * game.bid
        })
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
        .map(|(idx, game)| {
            (idx + 1) * game.bid
        })
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
