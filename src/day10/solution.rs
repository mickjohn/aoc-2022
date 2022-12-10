use super::parser;

const INPUT: &str = include_str!("input.txt");
const TEST_INPUT: &str = include_str!("test_input.txt");

pub fn solution() {
    println!("Solution for day ten part one = {}", part1(INPUT));
    println!("Solution for day ten part two:");
    part2(INPUT);
}

fn part1(input: &str) -> i32 {
    let instructions = parser::parse_input(input);
    let modified_instructions = modify_instructions(&instructions);

    let mut sum: i32 = 1;
    let mut signal_sum = 0;
    for (idx, instruction) in modified_instructions.iter().enumerate() {
        let clock = (idx as i32) + 1;
        if ((clock+20) % 40) == 0 {
            let signal = sum * clock;
            signal_sum += signal;
        }

        sum += instruction.value();
    }
    signal_sum
}

fn part2(input: &str) {
    let instructions = parser::parse_input(input);
    let modified_instructions = modify_instructions(&instructions);

    let mut sum: i32 = 1;
    let mut pixels: Vec<char> = Vec::with_capacity(220);
    for (idx, instruction) in modified_instructions.iter().enumerate() {
        let crt_pos = (idx as i32) % 40;
        if crt_pos.abs_diff(sum) <= 1 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }
        sum += instruction.value();
    }

    for (idx, c) in pixels.iter().enumerate() {
        if ((idx) % 40) == 0 {
            println!();
        }
        print!("{}", c);
    }
}

// Insert a noop before each Addx. (Now each Addx will take two clock cycles)
fn modify_instructions(instructions: &[Instruction]) -> Vec<Instruction> {
    let mut modified_instructions = Vec::new();
    instructions.iter().for_each(|i| match i {
        Instruction::Noop => modified_instructions.push(*i),
        Instruction::Addx(n) => {
            modified_instructions.push(Instruction::Noop);
            modified_instructions.push(Instruction::Addx(*n));
        }
    });
    modified_instructions
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    pub fn value(&self) -> i32 {
        match self {
            Instruction::Addx(x) => *x,
            _ => 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{part1, TEST_INPUT};



    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }

}

// struct Cpu {
//     pub clock: u32,
//     pub count: u32,
// }

// impl Cpu {
    
// }