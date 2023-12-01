advent_of_code::solution!(1);

fn char_to_u32(c: char) -> u32 {
    (c as u8 - b'0') as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().into_iter().fold(0, |acc, el| {
        let first_num = el.chars().find(char::is_ascii_digit).unwrap_or('0');
        let last_num = el.chars().rfind(char::is_ascii_digit).unwrap_or('0');
        let res: u32 = char_to_u32(first_num) * 10 + char_to_u32(last_num);

        acc + res
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    Some(input.lines().into_iter().fold(0, |acc, el| {
        let (min_i, min_pos) = nums
            .iter()
            .enumerate()
            .map(|(i, num)| (i, el.find(num)))
            .filter(|(_i, pos)| pos.is_some())
            .fold((None, usize::MAX), |(min_i, min_pos), (i, pos)| {
                let pos = pos.unwrap();
                if pos < min_pos {
                    (Some(i), pos)
                } else {
                    (min_i, min_pos)
                }
            });
        let (max_i, max_pos) = nums
            .iter()
            .enumerate()
            .map(|(i, num)| (i, el.rfind(num)))
            .filter(|(_i, pos)| pos.is_some())
            .fold((None, usize::MIN), |(max_i, max_pos), (i, pos)| {
                let pos = pos.unwrap();
                if pos > max_pos {
                    (Some(i), pos)
                } else {
                    (max_i, max_pos)
                }
            });
        let res: u32;
        if let Some(min_i_usize) = min_i {
            // We need to compare digit positions
            let maybe_first_num_pos = el.chars().position(|x| char::is_ascii_digit(&x));
            let maybe_last_num_pos = el
                .chars()
                .enumerate()
                .filter(|(_i, c)| char::is_ascii_digit(c))
                .last();
            let first_num: char;
            let last_num: char;
            if let Some(first_num_pos) = maybe_first_num_pos {
                let last_num_pos = maybe_last_num_pos.unwrap();
                if first_num_pos < min_pos {
                    first_num = el.chars().nth(first_num_pos).unwrap();
                } else {
                    first_num = (min_i_usize as u8 + b'1') as char;
                }
                if last_num_pos.0 > max_pos {
                    last_num = last_num_pos.1;
                } else {
                    last_num = (max_i.unwrap() as u8 + b'1') as char;
                }
            } else {
                first_num = (min_i_usize as u8 + b'1') as char;
                last_num = (max_i.unwrap() as u8 + b'1') as char;
            }
            res = char_to_u32(first_num) * 10 + char_to_u32(last_num);
        } else {
            // Just do the basic digits again
            let first_num = el.chars().find(char::is_ascii_digit).unwrap_or('0');
            let last_num = el.chars().rfind(char::is_ascii_digit).unwrap_or('0');
            res = char_to_u32(first_num) * 10 + char_to_u32(last_num);
        }

        acc + res
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
