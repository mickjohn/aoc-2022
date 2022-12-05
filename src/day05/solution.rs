use nom::{
  IResult,
  character::{complete::{char, alpha1, anychar}, is_alphabetic},
  sequence::{delimited, tuple},
  bytes::complete::is_not,
  bytes::complete::{tag, take, take_till, take_until},
  branch::alt, combinator::{map_res, opt},
};

#[derive(PartialEq, Debug)]
struct Crate(char);

impl From<&str> for Crate {
    fn from(s: &str) -> Self {
      assert_eq!(s.len(), 3);
      let chars: Vec<char> = s.chars().collect();
      Crate(chars[1])
    }
}

pub fn solution() {
  println!("Solution for day five part one ???");
  println!("Solution for day five part two ???");
}

fn parse(input: &str) {
}

fn foo() -> IResult<&'static str, &'static str> {
  let mut p = alt((tag("abcd"), tag("efgh")));
  p("fkljsdf")
}

fn parse_crate_char(input: &str) -> IResult<&str, Option<Crate>> {
  delimited(char('['), anychar, char(']') )(input)
  .map(|(i,o)| (i, Some(Crate(o))) )
}

fn parse_empty_crate(input: &str) -> IResult<&str, Option<Crate>> {
  tag("   ")(input).map(|(i,o)| (i, None) )
}

fn parse_crate(input: &str) -> IResult<&str, Option<Crate>> {
  alt((parse_crate_char, parse_empty_crate))(input)
}

fn parse_crate1(input: &str) -> IResult<&str, Option<Crate>> {
  tuple((
    alt((parse_crate_char, parse_empty_crate)),
    opt(whitespace)
  ))(input)
  .map(|(i, (o,_))| (i,o))
}

fn whitespace(input: &str) -> IResult<&str, &str> {
  alt((tag(" "), tag("\n")))(input)
}

fn parse_single_line(input: &str) -> IResult<&str, &str> {
  take_till(|c| c == '\n')(input)
}

fn parse_single_crate_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
  unimplemented!()
}


// fn parse_crate() -> FnMut(&str) -> IResult<&str, &str> {
//   delimited(char('['), is_not("]"), char(']'))
// }

// fn parse_delimiter(input: &str) -> IResult<&str, &str> {
//   tag("   ")
// }


#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

  #[test]
  fn test_crate_parser() {
    let (remainder1, output1) = super::parse_crate1("[a] ").unwrap();
    let (remainder2, output2) = super::parse_crate1("   \n").unwrap();
    assert_eq!(output1, Some(Crate('a')));
    assert!(remainder1.is_empty());
    assert_eq!(output2, None);
    assert!(remainder2.is_empty());
  }
}