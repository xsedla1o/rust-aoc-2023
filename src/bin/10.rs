use std::ops::{Index, IndexMut};

advent_of_code::solution!(10);

type Pos = (usize, usize);

struct Map<'a> {
    map: &'a [u8],
    width: usize,
}

impl<'a> Index<Pos> for Map<'a> {
    type Output = u8;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        &self.map[((self.width + 1) * y) + x]
    }
}

fn to_coords(map: &Map, position: usize) -> Pos {
    let y = position / (map.width + 1);
    let x = position % (map.width + 1);
    (x, y)
}

#[derive(Debug)]
enum D {
    Up,
    Down,
    Right,
    Left,
}

fn get_initial_moves(map: &Map, &start: &Pos) -> Vec<(D, Pos)> {
    let mut initial_moves: Vec<(D, Pos)> = Vec::new();
    let (x, y) = start;
    if y > 0 {
        let up_pos = (x, y - 1);
        if next_step(D::Up, map[up_pos], up_pos).is_some() {
            initial_moves.push((D::Up, up_pos));
        }
    }
    if x > 0 {
        let left_pos = (x - 1, y);
        if next_step(D::Left, map[left_pos], left_pos).is_some() {
            initial_moves.push((D::Left, left_pos));
        }
    }

    let down_pos = (x, y + 1);
    if next_step(D::Down, map[down_pos], down_pos).is_some() {
        initial_moves.push((D::Down, down_pos));
    }

    let right_pos = (x + 1, y);
    if next_step(D::Right, map[right_pos], right_pos).is_some() {
        initial_moves.push((D::Right, right_pos));
    }
    initial_moves
}

fn next_step(dir: D, tile: u8, (x, y): Pos) -> Option<(D, Pos)> {
    match (dir, tile) {
        (D::Up, b'|') => Some((D::Up, (x, y - 1))),
        (D::Down, b'|') => Some((D::Down, (x, y + 1))),

        (D::Left, b'-') => Some((D::Left, (x - 1, y))),
        (D::Right, b'-') => Some((D::Right, (x + 1, y))),

        (D::Down, b'L') => Some((D::Right, (x + 1, y))),
        (D::Left, b'L') => Some((D::Up, (x, y - 1))),

        (D::Down, b'J') => Some((D::Left, (x - 1, y))),
        (D::Right, b'J') => Some((D::Up, (x, y - 1))),

        (D::Up, b'7') => Some((D::Left, (x - 1, y))),
        (D::Right, b'7') => Some((D::Down, (x, y + 1))),

        (D::Up, b'F') => Some((D::Right, (x + 1, y))),
        (D::Left, b'F') => Some((D::Down, (x, y + 1))),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = input.as_bytes();
    let map = Map {
        map: bytes,
        width: bytes.iter().position(|&c| c == b'\n').unwrap(),
    };

    let start = bytes.iter().position(|&c| c == b'S').unwrap();
    let start = to_coords(&map, start);

    let mut initial_moves: Vec<(D, Pos)> = get_initial_moves(&map, &start);

    let mut state: (D, (usize, usize)) = initial_moves.pop().unwrap();
    let mut tile: u8 = map[state.1];
    let mut loop_length = 2;

    while tile != b'S' {
        state = next_step(state.0, tile, state.1).unwrap();
        tile = map[state.1];
        loop_length += 1;
    }

    Some(loop_length / 2)
}

struct VecMap<'a> {
    map: &'a mut Vec<u8>,
    width: usize,
}

impl<'a> Index<Pos> for VecMap<'a> {
    type Output = u8;

    fn index(&self, (x, y): Pos) -> &Self::Output {
        &self.map[((self.width + 1) * y) + x]
    }
}

impl<'a> IndexMut<Pos> for VecMap<'a> {
    fn index_mut(&mut self, (x, y): Pos) -> &mut Self::Output {
        &mut self.map[((self.width + 1) * y) + x]
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    return None;
    let bytes = input.as_bytes();
    let map = Map {
        map: bytes,
        width: bytes.iter().position(|&c| c == b'\n').unwrap(),
    };
    let mut loop_map = VecMap {
        map: &mut vec![0; bytes.len()],
        width: map.width,
    };

    let start = bytes.iter().position(|&c| c == b'S').unwrap();
    let start = to_coords(&map, start);
    let mut initial_moves = get_initial_moves(&map, &start);

    let mut state: (D, (usize, usize)) = initial_moves.pop().unwrap();
    let mut tile: u8 = map[state.1];
    loop_map[state.1] = 1;

    while tile != b'S' {
        state = next_step(state.0, tile, state.1).unwrap();
        tile = map[state.1];
        loop_map[state.1] = 1;
    }

    let mut col_sums: Vec<u32> = vec![0; loop_map.width];
    let mut enclosed = 0;

    let heigth = bytes.iter().filter(|&&x| x == b'\n').count();
    for y in 0..heigth {
        let mut row_sum: u32 = 0;
        for (x, col_sum) in col_sums.iter_mut().enumerate() {
            let val = loop_map[(x, y)];
            if val == 1 {
                *col_sum += 1;
                row_sum += 1;
            } else if row_sum % 2 != 0 || *col_sum % 2 != 0 {
                enclosed += 1;
            }
        }
    }

    Some(enclosed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_basic() {
        let result = part_one(
            ".....
.S-7.
.|.|.
.L-J.
.....
",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two(
            "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_harder() {
        let result = part_two(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
",
        );
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_hardest() {
        let result = part_two(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
",
        );
        assert_eq!(result, Some(10));
    }
}
