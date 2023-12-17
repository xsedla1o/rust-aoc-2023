use num::integer::lcm;
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

pub fn part_two(input: &str) -> Option<u64> {
    let (path, nodes) = input.split_once("\n\n").unwrap();
    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start_nodes = Vec::new();

    for node_mapping in nodes.lines() {
        let curr = &node_mapping[0..3];
        let left = &node_mapping[7..10];
        let right = &node_mapping[12..15];
        if curr.ends_with('A') {
            start_nodes.push(curr);
        }
        node_map.insert(curr, (left, right));
    }

    let total_steps = start_nodes
        .iter()
        .map(|state| node_map.get(state).unwrap())
        .map(|choice| {
            let mut curr_choice = choice;
            let mut steps: u64 = 0;
            loop {
                for action in path.chars() {
                    steps += 1;
                    let next_node = match action {
                        'L' => curr_choice.0,
                        'R' => curr_choice.1,
                        _ => panic!("Unknown action {action}"),
                    };

                    if next_node.ends_with('Z') {
                        return steps;
                    }

                    curr_choice = node_map.get(next_node).unwrap();
                }
            }
        })
        .fold(1, |acc, x| lcm(x, acc));

    Some(total_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, Some(6));
    }
}
