use core::fmt;
use std::cell::RefCell;

use nom::{IResult, bytes::complete::tag, character::complete::digit1, sequence::{tuple, terminated}, combinator::map_res, multi::separated_list1, branch::alt};

const INPUT: &str = include_str!("input.txt");

// Lowest common denominator for all of my divisible tests
const LCD: u64 = 9699690;


pub fn solution() {
    println!("Solution for day eleven part one = {}", part1()); 
    println!("Solution for day eleven part two = {}", part2()); 
}

fn part1() -> u64 {
    let mut monkeys = Monkeys {
        monkeys: parse_monkeys(INPUT).into_iter().map(RefCell::new).collect(),
        worry_divider: 3,
    };
    monkeys.simulate_rounds(20);
    monkeys.calculate_monkey_business()
}

fn part2() -> u64 {
    let mut monkeys = Monkeys {
        monkeys: parse_monkeys(INPUT).into_iter().map(RefCell::new).collect(),
        worry_divider: 1
    };
    monkeys.simulate_rounds(10000);
    monkeys.calculate_monkey_business()
}

#[derive(Clone, Copy, Debug)]
pub struct Test {
    test: u64,
    test_pass: usize,
    test_fail: usize,
}

impl Test {
    pub fn new(test: u64, test_pass: usize, test_fail: usize) -> Self {
        Self {
            test, test_pass, test_fail
        }
    }
}

#[derive(Debug, PartialEq)]
struct MonkeyMessage {
    pub monkey_number: usize,
    pub item: u64
}

impl MonkeyMessage {
    pub fn new(monkey_number: usize, item: u64) -> Self {
        Self { monkey_number, item }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    pub fn operate(&self, n: u64) -> u64 {
        use Operation::*;
        match *self {
            Add(x) => x+n,
            Multiply(x) => x*n,
            Square => n*n,
        }
    }
}


#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    test: Test,
    operation: Operation,
    inspections: u64,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "inspections: {}, items: {:?}", self.inspections, self.items)
    }
}

impl Monkey {

    pub fn throw_to(&self, worry: u64) -> usize {
        if (worry % self.test.test) == 0 {
            self.test.test_pass
        } else {
            self.test.test_fail
        }
    }

    pub fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }

    pub fn calculate_worry_for_each_item(&mut self, worry_divider: u64) -> Vec<MonkeyMessage> {
        (0..self.items.len()).map(|_| self.calculate_worry(worry_divider)).collect()
    }

    pub fn calculate_worry(&mut self, worry_divider: u64) -> MonkeyMessage {
        if self.items.is_empty() {
            panic!("Monkey has no items");
        }

        let item = self.items.pop().unwrap();
        let worry = (self.operation.operate(item)) / worry_divider;
        let reduced_worry = worry - ((worry/LCD) * LCD);
        let pass_to = self.throw_to(reduced_worry);
        self.inspections += 1;
        MonkeyMessage::new(pass_to, reduced_worry)
    }
}

struct Monkeys {
    pub monkeys: Vec<RefCell<Monkey>>,
    pub worry_divider: u64,
}

impl Monkeys {
    pub fn calculate_monkey_business(&mut self) -> u64 {
        self.monkeys.sort_by(|a,b| b.borrow().inspections.cmp(&a.borrow().inspections));
        self.monkeys[0].borrow().inspections * self.monkeys[1].borrow().inspections
    }

    pub fn simulate_rounds(&mut self, rounds: u32) {
        (0..rounds).for_each(|_| self.simulate_round());
    }

    fn simulate_round(&mut self) {
        for monkey in self.monkeys.iter() {
            let mut mut_mkey = monkey.borrow_mut();
            let messages = mut_mkey.calculate_worry_for_each_item(self.worry_divider);
            for message in messages {
                let mut target_monkey = self.monkeys[message.monkey_number].borrow_mut();
                target_monkey.add_item(message.item);
            }
        }
        // println!("{}\n-----------------", self);
    }
}

impl fmt::Display for Monkeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines: Vec<String> = self.monkeys.iter().enumerate().map(|(idx, m)| format!("Monkey {}: {}", idx, m.borrow())).collect();
        write!(f, "{}", lines.join("\n"))
    }
}


pub fn parse_monkey_line(input: &str) -> IResult<&str, ()> {
    tuple((
        tag("Monkey "),
        digit1,
        tag(":")
        
    ))(input).map(|(i, _)| (i, ()))
}

fn decimal(input: &str) -> IResult<&str, u64> {
  map_res(digit1, str::parse)(input)
}

fn parse_items_line(input: &str) -> IResult<&str, Vec<u64>> {
    tuple((
        tag("  Starting items: "),
        separated_list1(tag(", "), decimal)
    ))(input).map(|(i, (_, items))| (i, items))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    tuple((
        tag("  Operation: new = "),
        alt((parse_square, parse_add, parse_multiply))
    ))(input).map(|(i, (_, o))| (i, o))
}

fn parse_square(input: &str) -> IResult<&str, Operation> {
    tag("old * old")(input).map(|(i, _)| (i, Operation::Square))
}

fn parse_add(input: &str) -> IResult<&str, Operation> {
    tuple((
        tag("old + "),
        decimal
    ))(input).map(|(i, (_, num))| (i, Operation::Add(num)))
}

fn parse_multiply(input: &str) -> IResult<&str, Operation> {
    tuple((
        tag("old * "),
        decimal
    ))(input).map(|(i, (_, num))| (i, Operation::Multiply(num)))
}

fn parse_test(input: &str) -> IResult<&str, u64> {
    tuple((
        tag("  Test: divisible by "),
        decimal
    ))(input).map(|(i, (_, num))| (i, num))
}

fn parse_test_true(input: &str) -> IResult<&str, u64> {
    tuple((
        tag("    If true: throw to monkey "),
        decimal
    ))(input).map(|(i, (_, num))| (i, num))
}

fn parse_test_false(input: &str) -> IResult<&str, u64> {
    tuple((
        tag("    If false: throw to monkey "),
        decimal
    ))(input).map(|(i, (_, num))| (i, num))
}

fn parse_test_line(input: &str) -> IResult<&str, Test> {
    tuple((
        terminated(parse_test, tag("\n")),
        terminated(parse_test_true, tag("\n")),
        terminated(parse_test_false, tag("\n"))
    ))(input).map(|(i, (test, t, f))| (i, Test::new(test, t as usize, f as usize)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    tuple((
        terminated(parse_monkey_line, tag("\n")),
        terminated(parse_items_line, tag("\n")),
        terminated(parse_operation, tag("\n")),
        parse_test_line,
    ))(input).map(|(i, (_, items, operation, test))| {
        let m = Monkey {
            items,
            operation, 
            test,
            inspections: 0
        };
        (i, m)
    })
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let (_, monkeys) = separated_list1(tag("\n"), parse_monkey)(input).unwrap();
    monkeys

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_rounds() {
        let monkey_vec = vec![
            RefCell::new(Monkey {
                items: vec![98, 79],
                test: Test::new(23, 2, 3),
                operation: Operation::Multiply(19),
                inspections: 0,
            }),
            RefCell::new(Monkey {
                items: vec![74, 75, 65, 54],
                test: Test::new(19, 2, 0),
                operation: Operation::Add(6),
                inspections: 0,
            }),
            RefCell::new(Monkey {
                items: vec![97, 60, 79],
                test: Test::new(13, 1, 3),
                operation: Operation::Square,
                inspections: 0,
            }),
            RefCell::new(Monkey {
                items: vec![74],
                test: Test::new(17, 0, 1),
                operation: Operation::Add(3),
                inspections: 0
            })
        ];

        let mut monkeys = Monkeys { monkeys: monkey_vec, worry_divider: 3 };
        monkeys.simulate_rounds(20);
        assert_eq!(monkeys.calculate_monkey_business(), 10605);

    }

    #[test]
    fn test_parse_monkeys() {
        let monkeys = parse_monkeys(INPUT);
        let m = Monkeys {
            monkeys: monkeys.into_iter().map(|m| RefCell::new(m)).collect(),
            worry_divider: 3,
        };

        println!("{}", m);
        assert_eq!(m.monkeys.len(), 8);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(), 64032);
    }

}