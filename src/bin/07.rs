use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

fn card_to_index(card: char) -> usize {
    "23456789TJQKA".chars().position(|c| c == card).unwrap()
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
    let mut cards_in_hand: Vec<usize> = vec![0; 13];
    for card in hand.chars() {
        cards_in_hand[card_to_index(card)] += 1;
    }
    let mut sorted_hist = cards_in_hand
        .iter()
        .enumerate()
        .map(|(i, n)| (n, i))
        .sorted()
        .rev();
    let (count, _) = sorted_hist.next().unwrap();
    match count {
        5 => Hand::FiveoaK,
        4 => Hand::FouroaK,
        3 => {
            let (count, _) = sorted_hist.next().unwrap();
            if *count == 2 {
                Hand::FullHouse
            } else {
                Hand::ThreeoaK
            }
        }
        2 => {
            let (count, _) = sorted_hist.next().unwrap();
            if *count == 2 {
                Hand::TwoPairs
            } else {
                Hand::OnePair
            }
        }
        1 => Hand::HighCard,
        _ => panic!("Dont know what to do with count {count}"),
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
            .sorted()
            .enumerate()
            .map(|(i, play)| (play, i))
            .rev()
            .map(|(play, i)| play.bid * (i as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
