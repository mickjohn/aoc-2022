use nom::branch::alt;
use nom::{IResult, character::complete::digit1, combinator::map_res, sequence::tuple, bytes::complete::tag, multi::separated_list1};
use nom::character::complete::char;
use nom::combinator::opt;

use super::solution::Instruction;

pub fn parse_input(input: &str) -> Vec<Instruction> {
  let (_, instructions) = separated_list1(char('\n'), parse_line)(input).unwrap();
  instructions
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
  tag("noop")(input).map(|(i, _)| (i, Instruction::Noop))
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
  tuple((
    tag("addx "),
    signed_decimal
  ))(input).map(|(i, (_, num))| (i, Instruction::Addx(num)))

}

fn decimal(input: &str) -> IResult<&str, i32> {
  map_res(digit1, str::parse)(input)
}

fn signed_decimal(input: &str) -> IResult<&str, i32> {
  tuple((
    opt(char('-')),
    decimal
  ))(input)
  .map(|(i, (sign, num))| {
    let x = sign.map(|_| -num).unwrap_or(num);
    (i, x)
  })
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
  alt((
    parse_noop,
    parse_addx
  ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
      use super::Instruction::*;
      let lines = "addx 15\naddx -11\naddx 6\nnoop";
      let expected = vec![
        Addx(15),
        Addx(-11),
        Addx(6),
        Noop
      ];

      assert_eq!(parse_input(lines), expected);
    }
}