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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
