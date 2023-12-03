use itertools::Itertools;
use std::iter::zip;

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
                // println!("{l1}, {l2}");
                let mut prev_state = HeadState::default();
                let mut l1_state = HeadState::default();
                let mut l2_state = HeadState::default();
                let mut promote_state: LineState;

                for (i, (c1, c2)) in zip(l1.chars(), l2.chars()).enumerate() {
                    prev_state = prev_states[i];
                    l1_state.ctype = match c1 {
                        '.' => CharType::Void,
                        x if x.is_ascii_digit() => CharType::Digit,
                        _ => CharType::Symbol,
                    };
                    l2_state.ctype = match c2 {
                        '.' => CharType::Void,
                        x if x.is_ascii_digit() => CharType::Digit,
                        _ => CharType::Symbol,
                    };

                    let mut res = next_step(&mut l1_state, i, l1);
                    next_step(&mut l2_state, i, l2);

                    promote_state = LineState::None;
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
                        promote(&mut l1_state, &promote_state);
                    }

                    // println!("{i}, {:?}", &prev_state.state);
                    // println!("            {i}, {c1}, {:?}", &l1_state.state);
                    // println!(
                    //     "                             {i}, {c2}, {:?}",
                    //     &l2_state.state
                    // );

                    prev_states[i] = l1_state;

                    res += resolve_post(&mut l1_state, l1);
                    // if res != 0 {
                    //     println!("res: {res}");
                    // }
                    sum += res;
                    resolve_post(&mut l2_state, l2);
                }
                l1_state.ctype = CharType::Void;
                l2_state.ctype = CharType::Void;
                let mut res = next_step(&mut l1_state, size, l1);
                next_step(&mut l2_state, size, l2);

                promote_state = LineState::None;
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
                    promote(&mut l1_state, &promote_state);
                }

                res += resolve_post(&mut l1_state, l1);
                // if res != 0 {
                //     println!("res: {res}");
                // }
                sum += res;

                // println!();

                sum
            })
            .sum(),
    )
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
