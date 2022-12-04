use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

pub fn solution() {
    println!("Solution for day three part one is {}", part1());
    println!("Solution for day three part two is {}", part2());
}

fn part1() -> u32 {
    INPUT.split('\n')
        .map(|l| line_to_hashsets_with_values(l))
        .map(|(x,y)| {
            let v = x.intersection(&y).next().clone();
            v.map(|x| *x)
        })
        .filter(|n| n.is_some())
        .map(|n| n.unwrap())
        .sum()
}

fn part2() -> u32 {
    let lines: Vec<&str> = INPUT.split('\n').filter(|s| !s.is_empty()).collect();
    lines
        .as_slice()
        .chunks(3)
        .map(|window| {
            let three_lines: Vec<&str> = window.iter().map(|s| *s).collect();
            lines_to_common_value(&three_lines)
        }).sum()
}

fn line_to_hashsets_with_values(line: &str) -> (HashSet<u32>, HashSet<u32>) {
        let (first, second) = line.split_at((line.len()/2) as usize);
        let x: HashSet<u32> = str_to_value_set(first);
        let y: HashSet<u32> = str_to_value_set(second);
        (x,y)
}

fn lines_to_common_value(lines: &[&str]) -> u32 {
    let x = str_to_value_set(lines[0]);
    let y = str_to_value_set(lines[1]);
    let z = str_to_value_set(lines[2]);
    intersect(x, intersect(y, z)).iter().cloned().next().unwrap()
}

fn str_to_value_set(s: &str) -> HashSet<u32> {
    HashSet::from_iter(s.chars().map(to_value))
}

fn intersect(l: HashSet<u32>, r: HashSet<u32>) -> HashSet<u32> {
    l.intersection(&r).cloned().collect()
}

// https://www.utf8-chartable.de/
fn to_value(c: char) -> u32 {
    if c.is_lowercase() {
        // Subtract the UTF8 hex value to convert a-z to 1-26
        c as u32 - 0x61 + 1
    } else {
        // Subtract the UTF8 hex value, and add 26 to convert A-Z to 27-52
        c as u32 - 0x41 + 1 + 26
    }
}

