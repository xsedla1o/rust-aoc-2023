advent_of_code::solution!(2);

// Sum of game ids possible with only 12 red cubes, 13 green cubes, and 14 blue cubes
pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().into_iter().fold(0, |id_sum, line| {
        let (game, colors) = line.split_once(": ").unwrap();
        let game_id: u32 = game.split_once(' ').unwrap().1.parse().unwrap();
        for color_spec in colors.split("; ") {
            for cnt_color in color_spec.split(", ") {
                let (cnt, color) = cnt_color.split_once(' ').unwrap();
                let cnt: u32 = cnt.parse().unwrap();
                let limit = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Unknown color '{color}'"),
                };
                if cnt > limit {
                    return id_sum;
                }
            }
        }
        id_sum + game_id
    }))
}

// What is the sum of products of the minumum number of cubes required?
pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().into_iter().fold(0, |prod_sum, line| {
        let (_game, colors) = line.split_once(": ").unwrap();
        let mut r_min = 0;
        let mut g_min = 0;
        let mut b_min = 0;

        for color_spec in colors.split("; ") {
            for cnt_color in color_spec.split(", ") {
                let (cnt, color) = cnt_color.split_once(' ').unwrap();
                let cnt: u32 = cnt.parse().unwrap();
                match color {
                    "red" => r_min = if cnt > r_min { cnt } else { r_min },
                    "green" => g_min = if cnt > g_min { cnt } else { g_min },
                    "blue" => b_min = if cnt > b_min { cnt } else { b_min },
                    _ => panic!("Unknown color '{color}'"),
                };
            }
        }

        prod_sum + (r_min * g_min * b_min)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
