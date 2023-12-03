use itertools::Itertools;
use std::collections::VecDeque;

advent_of_code::solution!(3);

trait Intersects {
    fn instersects(&self, other: &Self) -> bool;
}

#[derive(Debug)]
struct NumBox {
    p: Rect,
    num: u32,
}

#[derive(Debug)]
struct Rect {
    y1: usize,
    x1: usize,
    y2: usize,
    x2: usize,
}

impl Intersects for Rect {
    fn instersects(&self, other: &Rect) -> bool {
        !(self.x2 < other.x1 || self.x1 > other.x2 || self.y2 < other.y1 || self.y1 > other.y2)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut nums: VecDeque<NumBox> = VecDeque::new();

    let mut last_line: (usize, &str) = (0, "");

    let l1 = input.lines().next().unwrap();
    collect_nums(&mut nums, (0, l1));

    for (l1, l2) in input.lines().enumerate().into_iter().tuple_windows() {
        collect_nums(&mut nums, l2);
        last_line = l2;

        let (y, line) = l1;
        remove_unrelevant(&mut nums, y);
        sum += sum_part_nums(&mut nums, y, line);
    }

    sum += sum_part_nums(&mut nums, last_line.0, last_line.1);

    Some(sum)
}

fn collect_nums(nums: &mut VecDeque<NumBox>, l: (usize, &str)) {
    let (y, line) = l;
    let mut collecting_number = false;
    let mut start_pos = 0;
    for (x, c) in line.chars().enumerate() {
        if !collecting_number && c.is_ascii_digit() {
            collecting_number = true;
            start_pos = x;
        } else if collecting_number && !c.is_ascii_digit() {
            collecting_number = false;
            let num = line[start_pos..x].parse::<u32>().unwrap();
            let pos = Rect {
                y1: y,
                x1: start_pos,
                y2: y,
                x2: x - 1,
            };
            let nb = NumBox { p: pos, num };
            nums.push_back(nb);
        }
    }
    if collecting_number {
        let num = line[start_pos..].parse::<u32>().unwrap();
        let pos = Rect {
            y1: y,
            x1: start_pos,
            y2: y,
            x2: line.len() - 1,
        };
        let nb = NumBox { p: pos, num };
        nums.push_back(nb);
    }
}

fn remove_unrelevant(nums: &mut VecDeque<NumBox>, y: usize) {
    while let Some(n) = nums.pop_front() {
        if n.p.y1 + 2 > y {
            nums.push_front(n);
            break;
        }
    }
}

fn sum_part_nums(nums: &mut VecDeque<NumBox>, y: usize, line: &str) -> u32 {
    let mut sum = 0;
    for (x, c) in line.chars().enumerate() {
        if c != '.' && !c.is_ascii_digit() {
            let symbol_pos = Rect {
                y1: if y > 0 { y - 1 } else { y },
                x1: if x > 0 { x - 1 } else { x },
                y2: (y + 1),
                x2: (x + 1),
            };
            for n in &mut *nums {
                if symbol_pos.instersects(&n.p) {
                    sum += n.num;
                }
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    let mut nums: VecDeque<NumBox> = VecDeque::new();
    let mut candidates: Vec<u32> = Vec::new();

    let mut last_line: (usize, &str) = (0, "");

    let l1 = input.lines().next().unwrap();
    collect_nums(&mut nums, (0, l1));

    for (l1, l2) in input.lines().enumerate().into_iter().tuple_windows() {
        collect_nums(&mut nums, l2);
        last_line = l2;

        let (y, line) = l1;
        remove_unrelevant(&mut nums, y);
        sum += sum_gear_ratios(&mut nums, y, line, &mut candidates);
    }

    sum += sum_gear_ratios(&mut nums, last_line.0, last_line.1, &mut candidates);

    Some(sum)
}

fn sum_gear_ratios(
    nums: &mut VecDeque<NumBox>,
    y: usize,
    line: &str,
    candidates: &mut Vec<u32>,
) -> u64 {
    let mut sum = 0;
    for (x, c) in line.chars().enumerate() {
        if c == '*' {
            let mul_pos = Rect {
                y1: if y > 0 { y - 1 } else { y },
                x1: if x > 0 { x - 1 } else { x },
                y2: (y + 1),
                x2: (x + 1),
            };
            candidates.clear();
            for n in &mut *nums {
                if mul_pos.instersects(&n.p) {
                    candidates.push(n.num);
                    if candidates.len() > 2 {
                        candidates.clear();
                        break;
                    }
                }
            }
            if candidates.len() == 2 {
                sum += candidates.iter().map(|x| *x as u64).product::<u64>();
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
