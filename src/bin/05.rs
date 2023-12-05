use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let mut chunks = input.split("\n\n");
    let mut nums: Vec<i64> = chunks
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut next_nums: Vec<i64> = Vec::with_capacity(nums.len());
    for chunk in chunks {
        let relevant_lines = chunk.split_once(":\n").unwrap().1;

        let ranges: Vec<_> = relevant_lines
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple::<(_, _, _)>()
                    .unwrap()
            })
            .collect();

        for num in nums.iter() {
            let mut found = false;
            for (dst_start, src_start, size) in ranges.iter() {
                let delta = dst_start - src_start;
                if src_start <= num && *num < src_start + size {
                    next_nums.push(*num + delta);
                    found = true;
                    break;
                }
            }
            if !found {
                next_nums.push(*num);
            }
        }
        nums = next_nums;
        next_nums = Vec::with_capacity(nums.len());
    }
    Some(*nums.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut chunks = input.split("\n\n");
    let mut num_iter = chunks
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|n| n.parse::<i64>().unwrap());
    let mut nums = Vec::new();
    while let Some(num) = num_iter.next() {
        let range_size = num_iter.next().unwrap();
        nums.push((num, range_size));
    }

    let mut next_nums = Vec::with_capacity(nums.len());
    for chunk in chunks {
        let relevant_lines = chunk.split_once(":\n").unwrap().1;
        let ranges: Vec<_> = relevant_lines
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple::<(_, _, _)>()
                    .unwrap()
            })
            .collect();

        for range in nums.iter() {
            let mut unsolved = VecDeque::from(vec![*range]);

            while let Some((num, num_cnt)) = unsolved.pop_front() {
                let mut found = false;
                let num_to = num + num_cnt;

                for (dst_start, src_start, size) in ranges.iter() {
                    let delta = dst_start - src_start;
                    let src_end = src_start + size;

                    // Begginning is in map
                    if *src_start <= num && num < src_end {
                        // Perfect overlap
                        if *src_start <= num
                            && num < src_end
                            && *src_start <= num_to
                            && num_to <= src_end
                        {
                            next_nums.push((num + delta, num_cnt));
                        } else {
                            next_nums.push((num + delta, src_end - num));
                            unsolved.push_back((src_end, num_to - src_end));
                        }
                        found = true;
                        break;
                    // End is in map
                    } else if *src_start < num_to && num_to < src_end {
                        next_nums.push((*dst_start, num_to - src_start));
                        unsolved.push_back((num, src_start - num));

                        found = true;
                        break;
                    // Map is inside the num range
                    } else if num <= *src_start
                        && *src_start <= num_to
                        && num <= src_end
                        && src_end <= num_to
                    {
                        next_nums.push((*dst_start, *size));
                        unsolved.push_back((num, src_start - num));
                        unsolved.push_back((src_end, num_to - src_end));
                        assert!(size + src_start - num + num_to - src_end == num_cnt);
                        found = true;
                        break;
                    }
                }
                if !found {
                    next_nums.push((num, num_cnt));
                }
            }
        }
        nums = next_nums;
        next_nums = Vec::with_capacity(nums.len());
    }
    Some(*nums.iter().map(|(num, _cnt)| num).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
