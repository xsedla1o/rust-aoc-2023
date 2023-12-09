use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

fn card_to_index(card: char) -> usize {
    match card {
        '2'..='9' => (card as u8 - b'2') as usize,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Unknown card {card}"),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeoaK,
    FullHouse,
    FouroaK,
    FiveoaK,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Play<'a> {
    hand: Hand,
    cards: &'a str,
    bid: u32,
}

impl<'a> PartialOrd for Play<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand.partial_cmp(&other.hand) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        for (my_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
            if my_card != other_card {
                return card_to_index(my_card).partial_cmp(&card_to_index(other_card));
            }
        }
        Some(Ordering::Equal)
    }
}

fn get_hand_kind(hand: &str) -> Hand {
    let mut cards_in_hand = [0; 13];
    for card in hand.chars() {
        cards_in_hand[card_to_index(card)] += 1;
    }
    cards_in_hand.sort_unstable_by(|a, b| b.cmp(a));
    match cards_in_hand[0] {
        5 => Hand::FiveoaK,
        4 => Hand::FouroaK,
        3 => {
            if cards_in_hand[1] == 2 {
                Hand::FullHouse
            } else {
                Hand::ThreeoaK
            }
        }
        2 => {
            if cards_in_hand[1] == 2 {
                Hand::TwoPairs
            } else {
                Hand::OnePair
            }
        }
        1 => Hand::HighCard,
        _ => panic!("Dont know what to do with count {}", cards_in_hand[0]),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands: Vec<Play> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            Play {
                hand: get_hand_kind(hand),
                cards: hand,
                bid: bid.parse().unwrap(),
            }
        })
        .collect();

    Some(
        hands
            .iter()
            .sorted_unstable()
            .enumerate()
            .map(|(i, play)| (play, i))
            .map(|(play, i)| play.bid * (i as u32 + 1))
            .sum(),
    )
}

fn card_to_index2(card: char) -> usize {
    match card {
        'J' => 0,
        '2'..='9' => (card as u8 - b'1') as usize,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Unknown card {card}"),
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Play2<'a> {
    hand: Hand,
    cards: &'a str,
    bid: u32,
}

impl<'a> PartialOrd for Play2<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand.partial_cmp(&other.hand) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        for (my_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
            if my_card != other_card {
                return card_to_index2(my_card).partial_cmp(&card_to_index2(other_card));
            }
        }
        Some(Ordering::Equal)
    }
}

fn get_hand_kind2(hand: &str) -> Hand {
    let mut cards_in_hand = [0; 13];
    let mut jokers = 0;
    for card in hand.chars() {
        if card == 'J' {
            jokers += 1
        } else {
            cards_in_hand[card_to_index2(card)] += 1;
        }
    }
    cards_in_hand.sort_unstable_by(|a, b| b.cmp(a));
    match cards_in_hand[0] + jokers {
        5 => Hand::FiveoaK,
        4 => Hand::FouroaK,
        3 => {
            if cards_in_hand[1] == 2 {
                Hand::FullHouse
            } else {
                Hand::ThreeoaK
            }
        }
        2 => {
            if cards_in_hand[1] == 2 {
                Hand::TwoPairs
            } else {
                Hand::OnePair
            }
        }
        1 => Hand::HighCard,
        _ => panic!("Dont know what to do with count {}", cards_in_hand[1]),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands: Vec<Play2> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            Play2 {
                hand: get_hand_kind2(hand),
                cards: hand,
                bid: bid.parse().unwrap(),
            }
        })
        .collect();

    Some(
        hands
            .iter()
            .sorted_unstable()
            .enumerate()
            .map(|(i, play)| (play, i))
            .map(|(play, i)| play.bid * (i as u32 + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
