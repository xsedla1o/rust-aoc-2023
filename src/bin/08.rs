use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (path, nodes) = input.split_once("\n\n").unwrap();
    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for node_mapping in nodes.lines() {
        let curr = &node_mapping[0..3];
        let left = &node_mapping[7..10];
        let right = &node_mapping[12..15];
        node_map.insert(curr, (left, right));
    }

    let mut curr_choice = node_map.get("AAA").unwrap();
    let mut steps = 0;
    'outer: loop {
        for action in path.chars() {
            let next = match action {
                'L' => curr_choice.0,
                'R' => curr_choice.1,
                _ => panic!("Unknown action {action}"),
            };
            steps += 1;
            if next == "ZZZ" {
                break 'outer;
            }
            curr_choice = node_map.get(next).unwrap();
        }
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
