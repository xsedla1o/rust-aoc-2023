use std::collections::VecDeque;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut winning: Vec<u8> = Vec::with_capacity(10);
    let sum = input
        .lines()
        .map(|line| {
            winning.clear();

            let mut card_sum = 0;
            let (_, card) = line.split_once(": ").unwrap();
            let (winning_nums, card_nums) = card.split_once(" | ").unwrap();

            for num in winning_nums.split(' ').filter(|num| !num.is_empty()) {
                winning.push(num.parse().unwrap());
            }

            for num in card_nums.split(' ').filter(|num| !num.is_empty()) {
                let num = num.parse().unwrap();
                if winning.contains(&num) {
                    if card_sum == 0 {
                        card_sum = 1;
                    } else {
                        card_sum *= 2;
                    }
                }
            }
            card_sum
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut winning: Vec<u8> = Vec::with_capacity(10);
    let mut copies: VecDeque<u32> = VecDeque::with_capacity(25);

    let sum = input
        .lines()
        .map(|line| {
            winning.clear();

            let card_cnt = copies.pop_front().unwrap_or(1);
            let mut card_sum = 0;
            let (_, card) = line.split_once(": ").unwrap();
            let (winning_nums, card_nums) = card.split_once(" | ").unwrap();

            for num in winning_nums.split(' ').filter(|num| !num.is_empty()) {
                winning.push(num.parse().unwrap());
            }

            for num in card_nums.split(' ').filter(|num| !num.is_empty()) {
                let num = num.parse().unwrap();
                if winning.contains(&num) {
                    card_sum += 1;
                }
            }

            for i in 0..card_sum {
                if let Some(elem) = copies.get_mut(i) {
                    *elem += card_cnt;
                } else {
                    copies.push_back(1 + card_cnt);
                }
            }
            card_cnt
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
