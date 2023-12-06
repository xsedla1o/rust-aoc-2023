advent_of_code::solution!(6);

fn parse_line(line: &str) -> Vec<u32> {
    line.split_at(11)
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times: Vec<_> = parse_line(lines.next().unwrap());
    let records: Vec<_> = parse_line(lines.next().unwrap());

    let result: usize = times
        .iter()
        .zip(records.iter())
        .map(|(time, record)| {
            (0..*time)
                .filter(|speed| (*time - speed) * speed > *record)
                .count()
        })
        .product();

    Some(result as u32)
}

fn parse_line2(line: &str) -> i64 {
    line.split_at(11).1.replace(' ', "").parse().unwrap()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut lines = input.lines();
    let time = parse_line2(lines.next().unwrap());
    let record = parse_line2(lines.next().unwrap());

    // `speed` or `hold time` is the `x`
    // (time - x) * x > record
    // time*x - x^2 - record > 0
    // -x^2 + time*x - record > 0

    let srtq_d = ((time * time - 4 * record) as f64).sqrt();
    let x1 = (((-time as f64) + srtq_d) / -2.0).floor() as i64;
    let x2 = (((-time as f64) - srtq_d) / -2.0).floor() as i64;

    Some(x2 - x1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
