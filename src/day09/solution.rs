use std::{collections::HashSet, hash::Hash};

use super::parser;

const INPUT: &str = include_str!("input.txt");
const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

pub fn solution() {
    println!("Solution for day nine part one = {}", part1(INPUT));
    println!("Solution for day nine part two = {}", part2(INPUT)); 
}

fn part1(input: &str) -> usize {
    let directions = parser::parse_input(input);
    let mut head: Point = Point::new(0, 0);
    let mut visited_by_head: Vec<Point> = Vec::new();

    directions.iter().for_each(|d| {
        head = head.follow_direction(*d);
        visited_by_head.push(head);
    });

    let mut tail: Point = Point::new(0, 0);
    let mut visited_by_tail: HashSet<Point> = HashSet::new();
    visited_by_tail.insert(tail);
    visited_by_head.iter().for_each(|head| {
        if !tail.is_adjacent(head) {
            tail = tail.move_to_adjacent(head);
            visited_by_tail.insert(tail);
        }
    });
    visited_by_tail.len()
}

fn part2(input: &str) -> usize {
    let directions = parser::parse_input(input);
    let visited_by_head = directions
        .iter()
        .fold((Point::new(0, 0), Vec::new()), |(p, mut v), d| {
            let new_p = p.follow_direction(*d);
            v.push(new_p);
            (new_p, v)
        })
        .1;

    let mut points = visited_by_head;
    (0..9).for_each(|_| points = follow_point(&points));
    HashSet::<Point>::from_iter(points).len()
}

fn follow_point(points: &[Point]) -> Vec<Point> {
    let mut tail: Point = Point::new(0, 0);
    let mut visited_by_tail: Vec<Point> = vec![tail];
    points.iter().for_each(|head| {
        if !tail.is_adjacent(head) {
            tail = tail.move_to_adjacent(head);
            visited_by_tail.push(tail);
        }
    });
    visited_by_tail
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        use Direction::*;
        match c {
            'U' => U,
            'D' => D,
            'L' => L,
            'R' => R,
            _ => panic!("Can only parse UDLR into Direction"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn follow_direction(&self, dir: Direction) -> Self {
        use Direction::*;
        match dir {
            U => Point::new(self.x, self.y + 1),
            D => Point::new(self.x, self.y - 1),
            L => Point::new(self.x - 1, self.y),
            R => Point::new(self.x + 1, self.y),
        }
    }

    pub fn is_adjacent(&self, other: &Point) -> bool {
        if self.y == other.y {
            (self.x - other.x).abs() <= 1 // Adjacent on the x axis
        } else if self.x == other.x {
            (self.y - other.y).abs() <= 1 // Adjacent on the y axis
        } else {
            ((self.x - other.x).abs() + (self.y - other.y).abs()) == 2 // Adjacent diagonally
        }
    }

    pub fn move_to_adjacent(&self, other: &Point) -> Point {
        match (other.x - self.x, other.y - self.y) {
            // Left Right movements
            (2, 0) => Point::new(other.x - 1, other.y),
            (-2, 0) => Point::new(other.x + 1, other.y),

            // Up Down movements
            (0, 2) => Point::new(other.x, other.y - 1),
            (0, -2) => Point::new(other.x, other.y + 1),

            // Part 2 Diagonal movements
            (2, 2) => Point::new(other.x - 1, other.y - 1),
            (2, -2) => Point::new(other.x - 1, other.y + 1),
            (-2, 2) => Point::new(other.x + 1, other.y - 1),
            (-2, -2) => Point::new(other.x + 1, other.y + 1),

            // Diagonal movements
            (_, 2) => Point::new(other.x, other.y - 1),
            (_, -2) => Point::new(other.x, other.y + 1),
            (2, _) => Point::new(other.x - 1, other.y),
            (-2, _) => Point::new(other.x + 1, other.y),
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_adjacent() {
        assert!(Point::new(0, 0).is_adjacent(&Point::new(0, 0)));
        assert!(Point::new(1, 0).is_adjacent(&Point::new(0, 0)));
        assert!(Point::new(0, 1).is_adjacent(&Point::new(0, 0)));
        assert!(Point::new(1, 1).is_adjacent(&Point::new(0, 0)));
        assert!(Point::new(-1, 0).is_adjacent(&Point::new(0, 0)));
        assert!(Point::new(0, -1).is_adjacent(&Point::new(0, 0)));
        assert!(Point::new(-1, -1).is_adjacent(&Point::new(0, 0)));

        assert!(!Point::new(0, 0).is_adjacent(&Point::new(2, 0)));
        assert!(!Point::new(0, 0).is_adjacent(&Point::new(2, 2)));
        assert!(!Point::new(0, 0).is_adjacent(&Point::new(0, -2)));
    }

    #[test]
    fn test_move_to_adjacent() {
        assert_eq!(
            Point::new(0, 0).move_to_adjacent(&Point::new(1, 0)),
            Point::new(0, 0)
        );

        assert_eq!(
            Point::new(0, 0).move_to_adjacent(&Point::new(2, 0)),
            Point::new(1, 0)
        );

        assert_eq!(
            Point::new(0, 0).move_to_adjacent(&Point::new(-2, 0)),
            Point::new(-1, 0)
        );

        assert_eq!(
            Point::new(0, 0).move_to_adjacent(&Point::new(0, 2)),
            Point::new(0, 1)
        );

        assert_eq!(
            Point::new(3, 0).move_to_adjacent(&Point::new(4, 2)),
            Point::new(4, 1)
        );
    }

    #[test]
    fn test_part_1() {
      assert_eq!(part1(TEST_INPUT), 13);
      assert_eq!(part1(INPUT), 6503);
    }

    #[test]
    fn test_part_2() {
      assert_eq!(part2(TEST_INPUT), 1);
      assert_eq!(part2(TEST_INPUT_2), 36);
      assert_eq!(part2(INPUT), 2724);
    }
}
