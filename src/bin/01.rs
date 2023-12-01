use regex::Regex;

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
    let pattern =
        Regex::new(r"([1-9])|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
            .unwrap();
    let pattern_rev =
        Regex::new(r"([1-9])|(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)")
            .unwrap();
    Some(input.lines().into_iter().fold(0, |acc, el| {
        let first = pattern.captures(el).unwrap();
        let mut first_num: u32 = 0;
        for i in 1..10 {
            if let Some(x) = first.get(i) {
                if i == 1 {
                    first_num = char_to_u32(x.as_str().chars().next().unwrap());
                    break;
                }
                first_num = i as u32 - 1;
                break;
            }
        }

        let reversed = el.chars().rev().collect::<String>();
        let first = pattern_rev.captures(&reversed).unwrap();
        let mut last_num: u32 = 0;
        for i in 1..=10 {
            if let Some(x) = first.get(i) {
                if i == 1 {
                    last_num = char_to_u32(x.as_str().chars().next().unwrap());
                    break;
                }
                last_num = i as u32 - 1;
                break;
            }
        }

        let res = first_num * 10 + last_num;
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
