use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    sequence::tuple, IResult,
};
use std::{collections::HashMap, hash::Hash};

const INPUT: &str = include_str!("input.txt");

pub fn solution() {
    println!("Solution for day twelve part one = {}", part1(INPUT));
    println!("Solution for day twelve part two = {}", part2(INPUT));
}

pub type Cave = HashMap<Point, Marker>;

fn get_lowest_point(points: &[Vec<Point>]) -> usize {
    let mut all_points: Vec<usize> = points.iter().flatten().map(|p| p.y).collect();
    all_points.sort_by(|a, b| b.cmp(a));
    all_points[0]
}

fn interpolate_point(start: Point, end: Point) -> Vec<Point> {
    let same_row = start.x == end.x;
    let row_in_order = start.y < end.y;
    let col_in_order = start.x < end.x;

    match same_row {
        true => match row_in_order {
            true => (start.y..=end.y).map(|y| Point::new(start.x, y)).collect(),
            false => (end.y..=start.y).map(|y| Point::new(start.x, y)).collect(),
        },
        false => match col_in_order {
            true => (start.x..=end.x).map(|x| Point::new(x, start.y)).collect(),
            false => (end.x..=start.x).map(|x| Point::new(x, start.y)).collect(),
        },
    }
}

fn interpolate_points(points: &[Vec<Point>]) -> Vec<Point> {
    points
        .iter()
        .flat_map(|line| {
            let new_points: Vec<Point> = line
                .windows(2)
                .flat_map(|window| interpolate_point(window[0], window[1]))
                .collect();
            new_points
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let coords = parse_all_input(input);

    // Interpolate the coordinate into lines of rock
    let full_coords = interpolate_points(&coords);
    let sand_start = Point::new(500, 0);
    let goal = get_lowest_point(&coords);
    let mut map: Cave = Cave::new();
    for point in full_coords {
        map.insert(point, Marker::Rock);
    }

    // Simulate the sand till a piece of sand passes the lowest rock
    while !simulate_sand(sand_start, goal, &mut map) {}
    map.values().filter(|v| **v == Marker::Sand).count()
}

fn part2(input: &str) -> usize {
    let mut coords = parse_all_input(input);
    let floor = get_lowest_point(&coords) + 2;

    // Add a really wide rock floor, 10000 wide shold be wide enough
    coords.push(vec![Point::new(0, floor), Point::new(10000, floor)]);

    // Interpolate the coordinate into lines of rock
    let full_coords = interpolate_points(&coords);
    let sand_start = Point::new(500, 0);
    let mut map: Cave = Cave::new();

    // Insert the rocks into the cave
    for point in full_coords {
        map.insert(point, Marker::Rock);
    }

    while !simulate_sand(sand_start, 999999, &mut map) {
        if map.contains_key(&sand_start) {
            break;
        }
    }
    map.values().filter(|v| **v == Marker::Sand).count()
}

fn simulate_sand(point: Point, goal_y: usize, map: &mut Cave) -> bool {
    if point.y >= goal_y {
        return true;
    }
    let (down, dleft, dright) = point.all();
    if is_empty(down, map) {
        simulate_sand(down, goal_y, map)
    } else if is_empty(dleft, map) {
        simulate_sand(dleft, goal_y, map)
    } else if is_empty(dright, map) {
        simulate_sand(dright, goal_y, map)
    } else {
        map.insert(point, Marker::Sand);
        false
    }
}

fn is_empty(s: Point, map: &Cave) -> bool {
    !map.contains_key(&s)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Marker {
    Rock,
    Sand,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn diag_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn diag_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    pub fn all(&self) -> (Self, Self, Self) {
        (self.down(), self.diag_left(), self.diag_right())
    }
}

fn decimal(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn parse_coordinate(input: &str) -> IResult<&str, Point> {
    tuple((decimal, tag(","), decimal))(input).map(|(i, (x, _, y))| (i, Point::new(x, y)))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(parse_arrow, parse_coordinate)(input)
}

fn parse_arrow(input: &str) -> IResult<&str, &str> {
    tag(" -> ")(input)
}

fn parse_all_input(input: &str) -> Vec<Vec<Point>> {
    let (_, output) = separated_list1(tag("\n"), parse_line)(input).unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{parse_all_input, Point};
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9
  "};

    #[test]
    fn test_parse_all_input() {
        let expected = vec![
            vec![Point::new(498, 4), Point::new(498, 6), Point::new(496, 6)],
            vec![
                Point::new(503, 4),
                Point::new(502, 4),
                Point::new(502, 9),
                Point::new(494, 9),
            ],
        ];

        let actual = parse_all_input(TEST_INPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_low_point() {
        let coords = parse_all_input(TEST_INPUT);
        assert_eq!(get_lowest_point(&coords), 9);
    }

    #[test]
    fn test_simulate_sand() {
        let coords = parse_all_input(TEST_INPUT);
        let full_coords = interpolate_points(&coords);
        let sand_start = Point::new(500, 0);
        let goal = get_lowest_point(&coords);
        let mut map = Cave::new();
        for point in full_coords {
            map.insert(point, Marker::Rock);
        }
        assert_eq!(map.get(&Point::new(500, 8)), None);
        simulate_sand(sand_start, goal, &mut map);
        simulate_sand(sand_start, goal, &mut map);
        simulate_sand(sand_start, goal, &mut map);
        simulate_sand(sand_start, goal, &mut map);
        simulate_sand(sand_start, goal, &mut map);
        assert_eq!(map.get(&Point::new(500, 8)), Some(&Marker::Sand));
        assert_eq!(map.get(&Point::new(499, 8)), Some(&Marker::Sand));
        assert_eq!(map.get(&Point::new(501, 8)), Some(&Marker::Sand));
        assert_eq!(map.get(&Point::new(499, 8)), Some(&Marker::Sand));
        assert_eq!(map.get(&Point::new(500, 7)), Some(&Marker::Sand));
    }

    #[test]
    fn test_part1_with_test_data() {
        let coords = parse_all_input(TEST_INPUT);
        let full_coords = interpolate_points(&coords);
        let sand_start = Point::new(500, 0);
        let goal = get_lowest_point(&coords);
        let mut map = Cave::new();
        for point in full_coords {
            map.insert(point, Marker::Rock);
        }

        let mut counter = 0;
        while !simulate_sand(sand_start, goal, &mut map) && counter < 30 {
            counter += 1;
        }
        let answer = map.values().filter(|v| **v == Marker::Sand).count();
        assert_eq!(answer, 24);
    }
}
