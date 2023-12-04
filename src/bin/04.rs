use std::collections::VecDeque;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(':').unwrap();
            let (winning_nums, card_nums) = card.split_once('|').unwrap();

            let win_cnt = card_nums
                .as_bytes()
                .chunks_exact(3)
                .filter(|num| {
                    winning_nums
                        .as_bytes()
                        .chunks_exact(3)
                        .any(|win_num| num == &win_num)
                })
                .count();

            2u32.pow(win_cnt as u32) >> 1
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut copies: VecDeque<u32> = VecDeque::with_capacity(25);

    let sum = input
        .lines()
        .map(|line| {
            let card_cnt = copies.pop_front().unwrap_or(1);
            let (_, card) = line.split_once(':').unwrap();
            let (winning_nums, card_nums) = card.split_once('|').unwrap();

            let card_sum = card_nums
                .as_bytes()
                .chunks_exact(3)
                .filter(|num| {
                    winning_nums
                        .as_bytes()
                        .chunks_exact(3)
                        .any(|win_num| num == &win_num)
                })
                .count();

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
