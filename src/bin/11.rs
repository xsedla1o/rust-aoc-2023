advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_cols = vec![1; input.chars().position(|c| c == '\n').unwrap()];
    let mut empty_rows_cumsum: Vec<u8> = Vec::new();

    let mut row_empty_cumsum = 0;
    for (y, line) in input.split('\n').enumerate() {
        let mut row_empty = 1;
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
            galaxies.push((x, y));
            empty_cols[x] = 0;
            row_empty = 0;
        }
        row_empty_cumsum += row_empty;
        empty_rows_cumsum.push(row_empty_cumsum);
    }

    let empty_cols_cumsum: Vec<u8> = empty_cols
        .into_iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    for (x, y) in galaxies.iter_mut() {
        *x += empty_cols_cumsum[*x] as usize;
        *y += empty_rows_cumsum[*y] as usize;
    }

    Some(
        galaxies
            .iter()
            .enumerate()
            .map(|(i, (x1, y1))| {
                galaxies
                    .iter()
                    .skip(i)
                    .fold(0, |acc, (x2, y2)| acc + x1.abs_diff(*x2) + y1.abs_diff(*y2))
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_cols = vec![1_000_000 - 1; input.chars().position(|c| c == '\n').unwrap()];
    let mut empty_rows_cumsum: Vec<usize> = Vec::new();

    let mut row_empty_cumsum = 0;
    for (y, line) in input.split('\n').enumerate() {
        let mut row_empty = 1_000_000 - 1;
        for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
            galaxies.push((x, y));
            empty_cols[x] = 0;
            row_empty = 0;
        }
        row_empty_cumsum += row_empty;
        empty_rows_cumsum.push(row_empty_cumsum);
    }

    let empty_cols_cumsum: Vec<usize> = empty_cols
        .into_iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    for (x, y) in galaxies.iter_mut() {
        *x += empty_cols_cumsum[*x];
        *y += empty_rows_cumsum[*y];
    }

    Some(
        galaxies
            .iter()
            .enumerate()
            .map(|(i, (x1, y1))| {
                galaxies
                    .iter()
                    .skip(i)
                    .fold(0, |acc, (x2, y2)| acc + x1.abs_diff(*x2) + y1.abs_diff(*y2))
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
