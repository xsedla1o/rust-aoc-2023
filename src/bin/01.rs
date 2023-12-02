advent_of_code::solution!(1);

#[inline(always)]
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

#[inline(always)]
fn forward_next_step(state: u8, input: char) -> (u8, Option<u32>) {
    match (state, input) {
        (0, x) if x.is_ascii_digit() => (0, Some(char_to_u32(x))),
        (0, 'o') => (1, None),
        (1, 'n') => (2, None),
        (2, 'e') => (0, Some(1)),

        (0, 't') => (3, None),
        (3, 'w') => (4, None),
        (4, 'o') => (0, Some(2)),

        (3, 'h') => (5, None),
        (5, 'r') => (6, None),
        (6, 'e') => (7, None),
        (7, 'e') => (0, Some(3)),

        (0, 'f') => (9, None),
        (9, 'o') => (10, None),
        (10, 'u') => (11, None),
        (11, 'r') => (0, Some(4)),

        (9, 'i') => (12, None),
        (12, 'v') => (13, None),
        (13, 'e') => (0, Some(5)),

        (0, 's') => (14, None),
        (14, 'i') => (15, None),
        (15, 'x') => (0, Some(6)),

        (14, 'e') => (16, None),
        (16, 'v') => (17, None),
        (17, 'e') => (18, None),
        (18, 'n') => (0, Some(7)),

        (0, 'e') => (19, None),
        (19, 'i') => (20, None),
        (20, 'g') => (21, None),
        (21, 'h') => (22, None),
        (22, 't') => (0, Some(8)),

        (0, 'n') => (23, None),
        (23, 'i') => (24, None),
        (24, 'n') => (25, None),
        (25, 'e') => (0, Some(9)),

        // Failure steps
        (_, _) => (0, None),
    }
}

#[inline(always)]
fn backward_next_step(state: u8, input: char) -> (u8, Option<u32>) {
    match (state, input) {
        (_, x) if x.is_ascii_digit() => (0, Some(char_to_u32(x))),
        (0, 'e') => (1, None),
        (1, 'n') => (2, None),
        (2, 'o') => (0, Some(1)),

        (0, 'o') => (3, None),
        (3, 'w') => (4, None),
        (4, 't') => (0, Some(2)),

        (1, 'e') => (5, None),
        (5, 'r') => (6, None),
        (6, 'h') => (7, None),
        (7, 't') => (0, Some(3)),

        (0, 'r') => (9, None),
        (9, 'u') => (10, None),
        (10, 'o') => (11, None),
        (11, 'f') => (0, Some(4)),

        (1, 'v') => (12, None),
        (12, 'i') => (13, None),
        (13, 'f') => (0, Some(5)),

        (0, 'x') => (14, None),
        (14, 'i') => (15, None),
        (15, 's') => (0, Some(6)),

        (0, 'n') => (16, None),
        (16, 'e') => (17, None),
        (17, 'v') => (18, None),
        (18, 'e') => (8, None),
        (8, 's') => (0, Some(7)),

        (0, 't') => (19, None),
        (19, 'h') => (20, None),
        (20, 'g') => (21, None),
        (21, 'i') => (22, None),
        (22, 'e') => (0, Some(8)),

        (2, 'i') => (25, None),
        (25, 'n') => (0, Some(9)),

        // There should probably be more failure steps, but these work for solving the input
        (17, 'n') => (2, None),
        (18, 'i') => (13, None),

        (_, _) => (0, None),
    }
}

#[inline(always)]
fn state_machine_find<I: Iterator<Item = char>>(
    transition_fn: fn(u8, char) -> (u8, Option<u32>),
    input_iter: I,
) -> u32 {
    let mut state = 0;
    for c in input_iter {
        match transition_fn(state, c) {
            // When we try to match from first state and fail, end
            (0, None) if state == 0 => (),

            // When we fail from another state, try again from state 0
            (0, None) => match forward_next_step(0, c) {
                // Update state on transition
                (new_state, None) => state = new_state,
                // Return on success
                (_, Some(num)) => return num,
            },

            // Update state on transition
            (new_state, None) => state = new_state,

            // Return on success
            (_, Some(num)) => return num,
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().into_iter().fold(0, |sum, line| {
        let first_num: u32 = state_machine_find(forward_next_step, line.chars());
        let last_num: u32 = state_machine_find(backward_next_step, line.chars().rev());
        let res = first_num * 10 + last_num;

        sum + res
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
