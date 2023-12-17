use itertools::Itertools;
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let nums: Vec<_> = line
                    .split(' ')
                    .map(|numstr| numstr.parse::<i32>().unwrap())
                    .collect();
                let mut nums_stack: Vec<Vec<_>> = vec![nums];

                loop {
                    let current_nums = nums_stack.last().unwrap();
                    let diffs = current_nums
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect_vec();

                    if !diffs.iter().all(|&x| x == 0) {
                        nums_stack.push(diffs);
                    } else {
                        break;
                    }
                }

                nums_stack
                    .iter()
                    .map(|series| series.last().unwrap())
                    .sum::<i32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let nums: Vec<_> = line
                    .split(' ')
                    .map(|numstr| numstr.parse::<i32>().unwrap())
                    .collect();
                let mut nums_stack: Vec<Vec<_>> = vec![nums];

                loop {
                    let current_nums = nums_stack.last().unwrap();
                    let diffs = current_nums
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect_vec();

                    if !diffs.iter().all(|&x| x == 0) {
                        nums_stack.push(diffs);
                    } else {
                        break;
                    }
                }

                nums_stack
                    .iter()
                    .map(|series| series.first().unwrap())
                    .rev()
                    .fold(0, |acc, x| x - acc)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
