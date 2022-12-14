use std::{fmt, collections::HashMap, hash::Hash, pin::Pin};


use nom::{bytes::complete::tag, IResult, combinator::map_res, character::complete::digit1, sequence::tuple, multi::separated_list1};

pub fn solution() {
    println!("Solution for day twelve part one = ???");
    println!("Solution for day twelve part two = ???");
}

type Cave = HashMap<Point, Marker>;

fn get_lowest_point(points: &Vec<Vec<Point>>) -> usize {
  let mut all_points: Vec<usize> = points.iter().flatten().map(|p| p.y).collect();
  all_points.sort_by(|a, b| b.cmp(a));
  all_points[0]
}

fn interpolate_points(points: &Vec<Vec<Point>>) -> &Vec<Vec<Point>> {
  for line in points {
    line.windows(2).map(|window| {
      let start = window[0];
      let end = window[1];
      if start.x == end.x {
        (start.y+1 .. end.y).map(|y| Point(start.x, y));
      }

    });
  }
  unimplemented!()
}

fn part1(input: &str) {
  let coords = parse_all_input(input);
  let sand_start = Point::new(500, 0);
  let goal = get_lowest_point(&coords);
  let mut map: HashMap<Point, Marker> = HashMap::new();
  let mut steps = 0;
  for line in coords {
    for coord in line {
      map.insert(coord, Marker::Rock);
    }
  }
  
  let mut sand = sand_start;
}

fn simulate_sand(point: Point, goal_y: usize, map: &mut Cave) {
  if point.y >= goal_y {
    println!("SUCCESS");
    return;
  }
  let (down, dleft, dright) = point.all();
  // Try move down
  if is_empty(down, map) {
    // Keep going
    println!("DOWN");
    simulate_sand(down, goal_y, map);
  } else if is_rock(down, map) {
    // If rock below, then settle some sand right here
    println!("STOP");
    map.insert(point, Marker::Sand);
  } else if is_sand(down, map) {
    // If sand below then try the diagonally adjacent points
    if is_empty(dleft, map) {
      // If spot to the left is empty, move sand there and keep going
      simulate_sand(dleft, goal_y, map)
    } else if is_empty(dright, map) {
      // If spot to the right is empty, move sand there and keep going
      simulate_sand(dright, goal_y, map)
    } else {
      // If neither side is empty, then put sand here!
      map.insert(point, Marker::Sand);
    }
  }
  // println!("NOTHING");
}

fn is_empty(s: Point, map: &Cave) -> bool {
  !map.contains_key(&s)
}

fn is_sand(s: Point, map: &Cave) -> bool {
  match map.get(&s) {
    Some(Marker::Sand) => true,
    _ => false,
  }
}

fn is_rock(s: Point, map: &Cave) -> bool {
  match map.get(&s) {
    Some(Marker::Rock) => true,
    _ => false,
  }
}

// fn is_rock_or_sand(s: Point, map: &Cave) -> bool {
//   is_rock(s, map) || is_rock_or_sand(s, map)
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Marker {
  Rock,
  Sand
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
  pub fn new(x: usize, y: usize) -> Self {
    Self {
      x,
      y
    }
  }

  pub fn down(&self) -> Self {
    Self {
      x: self.x,
      y: self.y + 1
    }
  }

  pub fn diag_left(&self) -> Self {
    Self {
      x: self.x - 1,
      y: self.y + 1,
    }
  }

  pub fn diag_right(&self) -> Self {
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
  tuple((
    decimal,
    tag(","),
    decimal,
  ))(input).map(|(i, (x, _, y))| (i, Point::new(x, y)))
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
  use indoc::indoc;
  use super::*;
  use super::{Point, parse_all_input};

  const TEST_INPUT: &str = indoc! {"
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9
  "};

  fn get_test_cave() -> Cave {
    let coords = parse_all_input(TEST_INPUT);
    let mut map: HashMap<Point, Marker> = HashMap::new();
    for line in coords {
      for coord in line {
        map.insert(coord, Marker::Rock);
      }
    }
    map
  }

  #[test]
  fn test_parse_all_input() {
    let expected = vec![
      vec![Point::new(498, 4), Point::new(498, 6), Point::new(496, 6)],
      vec![Point::new(503, 4), Point::new(502, 4), Point::new(502, 9), Point::new(494, 9)]
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
    let sand_start = Point::new(500, 0);
    let goal = get_lowest_point(&coords);
    let mut map: HashMap<Point, Marker> = HashMap::new();
    for line in coords {
      for coord in line {
        map.insert(coord, Marker::Rock);
      }
    }

    assert_eq!(map.get(&Point::new(500, 8)), None);
    simulate_sand(sand_start, goal, &mut map);
    println!("map after sim = {:?}", map);
    assert_eq!(map.get(&Point::new(500, 8)), Some(&Marker::Sand));
  }
}


