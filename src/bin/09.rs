advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut nums: Vec<_> = line
                    .split(' ')
                    .map(|numstr| numstr.parse::<i32>().unwrap())
                    .collect();

                let mut prediction = 0;
                let mut len = nums.len();
                let mut any_nonzero;

                loop {
                    any_nonzero = false;
                    for i in 0..len - 1 {
                        let res = nums[i + 1] - nums[i];
                        any_nonzero |= res != 0;
                        nums[i] = res;
                    }

                    len -= 1;
                    prediction += nums[len];

                    if !any_nonzero || len == 0 {
                        break;
                    }
                }

                prediction
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut nums: Vec<_> = line
                    .split(' ')
                    .map(|numstr| numstr.parse::<i32>().unwrap())
                    .collect();

                let mut prediction = 0;
                let len = nums.len();
                let mut start = 1;
                let mut any_nonzero;

                loop {
                    any_nonzero = false;
                    for i in (start..len).rev() {
                        let res = nums[i] - nums[i - 1];
                        any_nonzero |= res != 0;
                        nums[i] = res;
                    }

                    start += 1;

                    if !any_nonzero {
                        break;
                    }
                }

                for i in (0..start).rev() {
                    prediction = nums[i] - prediction;
                }

                prediction
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
