use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{
        complete::{anychar, char, digit1},
    },
    combinator::{map_res, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, tuple},
    IResult
};

const INPUT: &str = include_str!("input.txt");

pub fn solution() {
    println!("Solution for day five part one = {}", part1());
    println!("Solution for day five part two = {}", part2());
}

fn get_inputs() -> (CrateStack, Vec<Instruction>) {
  let (_, (crates, instructions)) = parse_puzzle_input(INPUT).unwrap();
  let mut crate_stack = CrateStack::new_from_crates(crates);
  (crate_stack, instructions)
}

fn part1() -> String {
  let (mut crate_stack, instructions) = get_inputs();
  instructions.iter().fold(&mut crate_stack, |acc, i| {
    acc.process_instruction_part1(i);
    acc
  });
  crate_stack.get_top_crates()
}

fn part2() -> String {
  let (mut crate_stack, instructions) = get_inputs();
  instructions.iter().fold(&mut crate_stack, |acc, i| {
    acc.process_instruction_part2(i);
    acc
  });
  crate_stack.get_top_crates()
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Crate(char);

impl From<&str> for Crate {
    fn from(s: &str) -> Self {
        assert_eq!(s.len(), 3);
        let chars: Vec<char> = s.chars().collect();
        Crate(chars[1])
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    pub from: u32,
    pub to: u32,
    pub num: u32,
}

#[derive(Debug)]
struct CrateStack {
    pub stacks: Vec<Vec<Crate>>,
}

impl CrateStack {
    pub fn process_instruction_part1(&mut self, instruction: &Instruction) {
      for _ in 0..instruction.num {
        let c = self.stacks[(instruction.from-1) as usize].pop().unwrap();
        self.stacks[(instruction.to-1) as usize].push(c);
      }
    } 
    
    pub fn process_instruction_part2(&mut self, instruction: &Instruction) {
      let mut temp = Vec::with_capacity(instruction.num as usize);
      for _ in 0..instruction.num {
        let c = self.stacks[(instruction.from-1) as usize].pop().unwrap();
        temp.push(c);
      }

      for c in temp.iter().rev() {
          self.stacks[(instruction.to-1) as usize].push(*c);
      }
    }

    pub fn new_from_crates(crates: Vec<Vec<Option<Crate>>>) -> Self {
      assert!(!crates.is_empty());
      let num_stacks = crates.len();
      let stack_depth = crates[0].len();
      let mut stacks: Vec<Vec<Crate>> = Vec::with_capacity(num_stacks as usize);

      // Prime the stacks
      crates[0].iter().for_each(|_| stacks.push(Vec::with_capacity(stack_depth as usize)));

      //Start pushing the crates onto the stack
      for row in crates.iter().rev() {
        for (idx, c) in row.iter().enumerate() {
          if c.is_some() {
            stacks[idx].push(c.unwrap());
          }
        }
      }

      Self { stacks }
    }

    pub fn get_top_crates(&self) -> String {
      self.stacks.iter().map(|s| s[s.len()-1].0).collect()
    }
}


// Parse a crate like [A]
fn parse_crate_char(input: &str) -> IResult<&str, Option<Crate>> {
    delimited(char('['), anychar, char(']'))(input).map(|(i, o)| (i, Some(Crate(o))))
}

// Parse the absense of a crate
fn parse_empty_crate(input: &str) -> IResult<&str, Option<Crate>> {
    tag("   ")(input).map(|(i, _)| (i, None))
}

// Parse either a crate, or the absense of a crate
fn parse_crate(input: &str) -> IResult<&str, Option<Crate>> {
    tuple((alt((parse_crate_char, parse_empty_crate)), opt(tag(" "))))(input)
        .map(|(i, (o, _))| (i, o))
}

// Parse all crates on a line
fn parse_single_crate_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    many1(parse_crate)(input)
}

// Parse all crates in the input
fn parse_all_crates(input: &str) -> IResult<&str, Vec<Vec<Option<Crate>>>> {
    separated_list1(char('\n'), parse_single_crate_line)(input)
}

// Parse a decimal number
fn decimal(input: &str) -> IResult<&str, u32> {
  map_res(digit1, str::parse)(input)
}

// Parse instruction
fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    tuple((
        tag("move "),
        decimal,
        tag(" from "),
        decimal,
        tag(" to "),
        decimal,
    ))(input)
    .map(|(input, (_, num, _, from, _, to))| (input, Instruction { from, to, num }))
}

// Parse all of the instructions
fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
  separated_list1(char('\n'), parse_instruction)(input)
}

// Parse the useless line with numbers, i.e. ' 1  2  3  4 etc'
fn parse_number_line(input: &str) -> IResult<&str, ()> {
  tuple((
    take_till(|c| c == '\n'),
    tag("\n")
  ))(input).map(|(i, _)| (i, ()))
}

// Parse the puzzle input into a 2d vector of crates, and a list of instructions
fn parse_puzzle_input(input: &str) -> IResult<&str, (Vec<Vec<Option<Crate>>>, Vec<Instruction>)> {
  tuple((
    parse_all_crates,
    tag("\n"),
    parse_number_line,
    char('\n'),
    parse_instructions
  ))(input).map(|(i, (crates, _, _, _, instructions))| {
    (i, (crates, instructions))
  })
}

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
        let (remainder1, output1) = super::parse_crate("[a] ").unwrap();
        let (remainder2, output2) = super::parse_crate("   \n").unwrap();
        assert_eq!(output1, Some(Crate('a')));
        assert!(remainder1.is_empty());
        assert_eq!(output2, None);
        assert!(!remainder2.is_empty());
    }

    #[test]
    fn test_crates_parser() {
        let table = vec![
            // input, expected remainder, expected output
            (
                "[a] [b] [c]",
                "",
                vec![Some(Crate('a')), Some(Crate('b')), Some(Crate('c'))],
            ),
            (
                "[a]     [c]",
                "",
                vec![Some(Crate('a')), None, Some(Crate('c'))],
            ),
            ("        [c]", "", vec![None, None, Some(Crate('c'))]),
        ];

        for (input, expected_remainder, expected_output) in table {
            let (remainder, output) = parse_single_crate_line(input).unwrap();
            assert_eq!(remainder, expected_remainder);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_parse_all_crates() {
        let expected = vec![
            vec![None, Some(Crate('D')), None],
            vec![Some(Crate('N')), Some(Crate('C')), None],
            vec![Some(Crate('Z')), Some(Crate('M')), Some(Crate('P'))],
        ];

        let expected_remainder = "
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let (remainder, output) = parse_all_crates(TEST_INPUT).unwrap();
        assert_eq!(output, expected);
        assert_eq!(remainder, expected_remainder);
    }

    #[test]
    fn test_parse_number_line() {
      let (remainder, _) = parse_number_line(" 1   2   3 \n").unwrap();
      assert!(remainder.is_empty());
    }

    #[test]
    fn test_parse_instruction() {
      let table = vec![
        ("move 1 from 2 to 1", Instruction{num: 1, from: 2, to: 1}),
        ("move 10 from 20 to 10", Instruction{num: 10, from: 20, to: 10}),
      ];

      for (input, expected) in table {
        let (_remainder, instruction) = parse_instruction(input).unwrap();
        assert_eq!(instruction, expected);
      }
    }

    #[test]
    fn test_parse_instructions() {
      let input = "move 1 from 2 to 1
move 100 from 2 to 1
move 9 from 88 to 2";
      let expected = vec![
        Instruction{num: 1, from: 2, to: 1},
        Instruction{num: 100, from: 2, to: 1},
        Instruction{num: 9, from: 88, to: 2},
      ];

      let (_remainder, parsed) = parse_instructions(input).unwrap();
      assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_puzzle_input() {
      let expected_crates = vec![
        vec![None, Some(Crate('D')), None],
        vec![Some(Crate('N')), Some(Crate('C')), None],
        vec![Some(Crate('Z')), Some(Crate('M')), Some(Crate('P'))],
      ];

      let expected_instructions = vec![
        Instruction{num: 1, from: 2, to: 1},
        Instruction{num: 3, from: 1, to: 3},
        Instruction{num: 2, from: 2, to: 1},
        Instruction{num: 1, from: 1, to: 2},
      ];

      let (_remainder, (crates, instructions)) = parse_puzzle_input(TEST_INPUT).unwrap();
      assert_eq!(crates, expected_crates);
      assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn test_create_crate_stack() {
      let crates = vec![
        vec![None, Some(Crate('D')), None],
        vec![Some(Crate('N')), Some(Crate('C')), None],
        vec![Some(Crate('Z')), Some(Crate('M')), Some(Crate('P'))],
      ];

      let crate_stack = CrateStack::new_from_crates(crates);
      assert_eq!(crate_stack.stacks[0], vec![Crate('Z'), Crate('N')]);
      assert_eq!(crate_stack.stacks[1], vec![Crate('M'), Crate('C'), Crate('D')]);
      assert_eq!(crate_stack.stacks[2], vec![Crate('P')]);
    }
    
    #[test]
    fn test_instructions() {
      let crates = vec![
        vec![None, Some(Crate('D')), None],
        vec![Some(Crate('N')), Some(Crate('C')), None],
        vec![Some(Crate('Z')), Some(Crate('M')), Some(Crate('P'))],
      ];

      let mut crate_stack = CrateStack::new_from_crates(crates);
      let instructions = vec![
        Instruction{num: 1, from: 2, to: 1},
        Instruction{num: 3, from: 1, to: 3},
        Instruction{num: 2, from: 2, to: 1},
        Instruction{num: 1, from: 1, to: 2},
      ];

      for i in instructions {
        crate_stack.process_instruction_part1(&i);
      }
      println!("{:?}", crate_stack);
      assert_eq!(crate_stack.get_top_crates(), "CMZ");
    }
    
    #[test]
    fn test_instructions_part2() {
      let crates = vec![
        vec![None, Some(Crate('D')), None],
        vec![Some(Crate('N')), Some(Crate('C')), None],
        vec![Some(Crate('Z')), Some(Crate('M')), Some(Crate('P'))],
      ];

      let mut crate_stack = CrateStack::new_from_crates(crates);
      let instructions = vec![
        Instruction{num: 1, from: 2, to: 1},
        Instruction{num: 3, from: 1, to: 3},
        Instruction{num: 2, from: 2, to: 1},
        Instruction{num: 1, from: 1, to: 2},
      ];

      for i in instructions {
        crate_stack.process_instruction_part2(&i);
      }
      println!("{:?}", crate_stack);
      assert_eq!(crate_stack.get_top_crates(), "MCD");
    }

}
