use std::cmp::Ordering;

#[derive(Debug, PartialOrd, Ord, Clone)]
#[repr(u8)]
pub(crate) enum Card {
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
    pub(crate) fn from_char(c: char, use_joker: bool) -> Card {
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
pub(crate) enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
pub(crate) struct Hand {
    pub(crate) cards: Vec<Card>,
    pub(crate) hand_type: HandType,
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
pub(crate) struct Game {
    pub(crate) hand: Hand,
    pub(crate) bid: usize,
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

pub(crate) trait HandTypeDeterminer {
    fn determine_hand_type(&self) -> HandType;
}

impl HandTypeDeterminer for Vec<Card> {
     fn determine_hand_type(&self) -> HandType {
        let mut cards = self.to_vec();

        let the_joker = Card::Number(0);

        cards.sort();

        let is_five_of_kind = {
            cards[0] == cards[4] || (
                    // Jokers
                    // JJJJ<other>
                    cards[3] == the_joker

                    // JJJ<other><other>
                    || (cards[2] == the_joker && cards[3] == cards[4] )

                    // JJ<other><other><other>
                    || (cards[1] == the_joker && cards[2] == cards[4] )

                    // J<other><other><other><other>
                    || (cards[0] == the_joker && cards[1] == cards[4] ))
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
                    cards[2] == the_joker

                    // JJ<other><other><other>
                    || (cards[1] == the_joker && (cards[2] == cards[3] || cards[3] == cards[4] ) )

                    // J<other><other><other><other>
                    || (cards[0] == the_joker && (cards[1] == cards[3] || cards[2] == cards[4]))
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
            || (*second == the_joker && (fourth == fifth))

            // J<other><other><other><other>
            || (*first == the_joker &&  ((second == third && fourth == fifth) ))
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
            ||  *second == the_joker 

            // J<other><other><other><other>
            || (*first == the_joker && (second == third || third == fourth || fourth == fifth ))
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
            (*first == the_joker && (second == third || second == fourth || second == fifth || third == fourth || third == fifth || fourth == fifth))
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
                || (*first == the_joker)
        };

        if is_one_pair {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

