use nom::{IResult, character::complete::{one_of, digit1, anychar}, Parser, combinator::map_res, sequence::tuple, bytes::complete::tag, multi::separated_list1};
use nom::character::complete::char;

use super::solution::Direction;

pub fn parse_input(input: &str) -> Vec<Direction> {
  let (_, dirs) = separated_list1(char('\n'), parse_line)(input).unwrap();
  dirs.concat()
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
  one_of("UDLR")(input).map(|(i, c)| (i, Direction::from(c)))
}

fn decimal(input: &str) -> IResult<&str, u32> {
  map_res(digit1, str::parse)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Direction>> {
  tuple((parse_direction, tag(" "), decimal))(input)
  .map(|(i, (d, _, n))| {
    let directions: Vec<Direction> = (0..n).map(|_| d).collect();
    (i, directions)
  })
}

#[cfg(test)]
mod tests {
    use super::parse_input;

  const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_parse() {
      use super::super::solution::Direction::*;

      let dirs = parse_input(&TEST_INPUT);
      let expected = vec![R,R,R,R,U,U,U,U,L,L,L,D,R,R,R,R,D,L,L,L,L,L,R,R];
      assert_eq!(dirs, expected);
    }

}