use itertools::Itertools;
use std::{collections::VecDeque, iter::zip};

advent_of_code::solution!(3);

#[derive(Debug, PartialEq, Clone, Copy)]
enum LineState {
    None,
    Num,
    ValidNum,
    ValidNumSymbol,
    NumPost,
    ValidNumPost,
    Symbol,
    SymbolPost,
}

impl Default for LineState {
    fn default() -> Self {
        LineState::None
    }
}

#[derive(Clone, Copy)]
enum CharType {
    Void,
    Digit,
    Symbol,
}

impl Default for CharType {
    fn default() -> Self {
        CharType::Void
    }
}

#[derive(Default, Clone, Copy)]
struct HeadState {
    state: LineState,
    ctype: CharType,
    num_start: usize,
    num_end: usize,
}

fn next_step(mut s: &mut HeadState, i: usize, line: &str) -> u32 {
    let mut sum = 0;
    match (&s.state, &s.ctype) {
        (&LineState::None, CharType::Digit) => {
            s.state = LineState::Num;
            s.num_start = i;
        }
        (&LineState::Symbol, CharType::Digit) => {
            s.state = LineState::ValidNumSymbol;
            s.num_start = i;
        }
        (&LineState::ValidNumSymbol, CharType::Digit) => {
            s.state = LineState::ValidNum;
        }
        (&LineState::None, CharType::Symbol) => {
            s.state = LineState::Symbol;
        }
        (&LineState::Num | &LineState::ValidNum | &LineState::ValidNumSymbol, CharType::Symbol) => {
            sum += line[s.num_start..i].parse::<u32>().unwrap();
            s.state = LineState::Symbol
        }
        (
            st @ (&LineState::Num | &LineState::ValidNum | &LineState::ValidNumSymbol),
            CharType::Void,
        ) => {
            s.state = match *st {
                LineState::Num => LineState::NumPost,
                LineState::ValidNum => LineState::ValidNumPost,
                LineState::ValidNumSymbol => LineState::ValidNumPost,
                _ => panic!("{st:?}"),
            };
            s.num_end = i;
        }
        (&LineState::Symbol, CharType::Void) => s.state = LineState::SymbolPost,
        (&LineState::SymbolPost, CharType::Void) => s.state = LineState::None,
        (&LineState::SymbolPost, CharType::Symbol) => s.state = LineState::Symbol,
        (&LineState::ValidNumPost, _) => {
            sum += line[s.num_start..s.num_end].parse::<u32>().unwrap();
            s.state = LineState::None
        }
        _ => (),
    }
    sum
}

fn promote(mut s: &mut HeadState, ls: &LineState) {
    s.state = match &s.state {
        LineState::Num => LineState::ValidNum,
        LineState::NumPost if ls == &LineState::Symbol => LineState::ValidNumPost,
        &x => x,
    }
}

fn resolve_post(mut s: &mut HeadState, line: &str) -> u32 {
    let mut sum = 0;
    match (&s.state, &s.ctype) {
        (&LineState::ValidNumPost, _) => {
            sum += line[s.num_start..s.num_end].parse::<u32>().unwrap();
            s.state = LineState::None
        }
        (_, CharType::Void) => s.state = LineState::None,
        _ => (),
    }
    sum
}

fn execute_promote(prev_state: &HeadState, l1_state: &mut HeadState, l2_state: &HeadState) {
    let mut promote_state = LineState::None;
    if l1_state.state == LineState::SymbolPost
        || l2_state.state == LineState::SymbolPost
        || prev_state.state == LineState::SymbolPost
        || l1_state.state == LineState::ValidNumSymbol
        || l2_state.state == LineState::ValidNumSymbol
        || prev_state.state == LineState::ValidNumSymbol
    {
        promote_state = LineState::SymbolPost;
    }
    if l1_state.state == LineState::Symbol
        || l2_state.state == LineState::Symbol
        || prev_state.state == LineState::Symbol
    {
        promote_state = LineState::Symbol;
    }
    if promote_state != LineState::None {
        promote(l1_state, &promote_state);
    }
}

fn char_to_state(c: char) -> CharType {
    match c {
        '.' => CharType::Void,
        x if x.is_ascii_digit() => CharType::Digit,
        _ => CharType::Symbol,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let size = input.lines().next().unwrap().len();
    let mut prev_states: Vec<HeadState> = vec![HeadState::default(); size];
    let input: String = input.to_string() + &".".repeat(size) + "\n";
    Some(
        input
            .lines()
            .into_iter()
            .tuple_windows()
            .map(|(l1, l2)| {
                let mut sum = 0;
                let mut prev_state = HeadState::default();
                let mut l1_state = HeadState::default();
                let mut l2_state = HeadState::default();

                for (i, (c1, c2)) in zip(l1.chars(), l2.chars()).enumerate() {
                    prev_state = prev_states[i];
                    l1_state.ctype = char_to_state(c1);
                    l2_state.ctype = char_to_state(c2);

                    let mut res = next_step(&mut l1_state, i, l1);
                    next_step(&mut l2_state, i, l2);

                    execute_promote(&prev_state, &mut l1_state, &l2_state);

                    prev_states[i] = l1_state;

                    res += resolve_post(&mut l1_state, l1);
                    sum += res;
                    resolve_post(&mut l2_state, l2);
                }
                l1_state.ctype = CharType::Void;
                l2_state.ctype = CharType::Void;
                let mut res = next_step(&mut l1_state, size, l1);
                next_step(&mut l2_state, size, l2);

                execute_promote(&prev_state, &mut l1_state, &l2_state);

                res += resolve_post(&mut l1_state, l1);
                sum += res;
                sum
            })
            .sum(),
    )
}

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

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    let mut nums: VecDeque<NumBox> = VecDeque::new();
    for (y, line) in input.lines().enumerate() {
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

    let mut candidates: Vec<u32> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        while let Some(n) = nums.pop_front() {
            if n.p.y1 + 2 > y {
                nums.push_front(n);
                break;
            }
        }
        let relevant: Vec<&NumBox> = nums
            .iter()
            .filter(|n| n.p.y1 + 1 >= y && n.p.y2 <= y + 1)
            .collect();
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                let mul_pos = Rect {
                    y1: if y > 0 { y - 1 } else { y },
                    x1: if x > 0 { x - 1 } else { x },
                    y2: (y + 1),
                    x2: (x + 1),
                };
                candidates.clear();
                for n in &relevant {
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
    }
    Some(sum)
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
