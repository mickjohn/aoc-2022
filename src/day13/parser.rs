use std::{rc::Rc, cell::RefCell};

use nom::{combinator::map_res, IResult, character::complete::digit1, multi::{separated_list1, separated_list0, many1}, bytes::complete::{tag, is_a}, branch::alt};

use super::solution::{Data, Packet};

#[derive(PartialEq, PartialOrd, Debug)]
enum Token {
    ArrStart,
    ArrEnd,
    Num(i32),
    Comma,
}

#[derive(PartialEq, Debug)]
struct Node {
//   pub nums: RefCell<Vec<i32>>,
  pub nums: Vec<i32>,
  pub nodes: Vec<Rc<RefCell<Node>>>,
  pub parent: Option<Rc<RefCell<Node>>>,
}

fn parse_arr_start(input: &str) -> IResult<&str, Token> {
    tag("[")(input).map(|(i, _)| (i, Token::ArrStart))
}

fn parse_arr_end(input: &str) -> IResult<&str, Token> {
    tag("]")(input).map(|(i, _)| (i, Token::ArrEnd))
}

fn parse_comma(input: &str) -> IResult<&str, Token> {
    tag(",")(input).map(|(i, _)| (i, Token::Comma))
}

fn parse_num(input: &str) -> IResult<&str, Token> {
    decimal(input).map(|(i, num)| (i, Token::Num(num)))
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        parse_arr_end,
        parse_arr_start,
        parse_comma,
        parse_num
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Token>> {
    many1(parse_token)(input)
}

fn decimal(input: &str) -> IResult<&str, i32> {
  map_res(digit1, str::parse)(input)
}

fn build_tree(lines: Vec<Token>) -> Rc<RefCell<Node>> {
  let root = Rc::new(RefCell::new(Node {nums: Vec::new(), nodes: Vec::new(), parent: None }));
  let mut current_node: Rc<RefCell<Node>> = Rc::clone(&root);

  for line in lines {
    match line {
        Token::Num(x) => current_node.borrow_mut().nums.push(x),
        Token::ArrStart => {
            let child = Rc::new(RefCell::new(Node {nums: Vec::new(), nodes: Vec::new(), parent: Some(current_node.clone())}));
            current_node.borrow_mut().nodes.push(child.clone());
            current_node = child;
        }
        Token::ArrEnd => {
            let parent = Rc::clone(current_node.borrow().parent.as_ref().unwrap());
            current_node = parent;
        }
        Token::Comma => (),


    //   Line::Cd(dir) if dir == ".." => {
    //     let parent = Rc::clone(current_dir.borrow().parent.as_ref().unwrap());
    //     current_dir = parent;
    //   }
    //   Line::Cd(dir) => {
    //     let child = Rc::clone(current_dir.borrow().dirs.iter().find(|d| d.borrow().name == dir).as_ref().unwrap());
    //     current_dir = child;
    //   },
    //   Line::FileLine(f) => {
    //     current_dir.borrow_mut().files.push(Rc::new(RefCell::new(f)))
    //   },
    //   Line::DirLine(mut d) => {
    //     d.parent = Some(Rc::clone(&current_dir));
    //     current_dir.borrow_mut().dirs.push(Rc::new(RefCell::new(d)))
    //   },
    }
  }
  root
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::day13::{parser::parse_line, solution::Packet};

    #[test]
    fn test_parse_line() {
        use Token::*;
        let expected = vec![ArrStart, ArrStart, Num(1), ArrEnd, Comma, ArrStart, Num(2), Comma, Num(3), Comma, Num(4), ArrEnd, ArrEnd];
        let (_, actual) = parse_line("[[1],[2,3,4]]").unwrap();
        assert_eq!(expected, actual);
    }
}
