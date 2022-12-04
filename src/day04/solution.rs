const INPUT: &str = include_str!("input.txt");



struct Assignment {
    pub start: u32,
    pub end: u32,
}

pub fn solution() {
    let lines: Vec<&str> = INPUT.split('\n').filter(|s| !s.is_empty()).collect();
    println!("Solution for day four part one {}", part1(&lines));
    println!("Solution for day four part two {}", part2(&lines));
}

impl Assignment {
    pub fn overlaps_with_fully(&self, other: &Self) -> bool {
        (self.start >= other.start && self.end <= other.end)
            || (self.start <= other.start && self.end >= other.end)
    }

    fn check_overlap(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
        || (self.end >= other.start && self.end <= other.end)
    }

    pub fn overlaps_with_at_all(&self, other: &Self) -> bool {
        self.check_overlap(other) || other.check_overlap(self)
    }
}

impl TryFrom<&str> for Assignment {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Error parsing.");
        }
        let start = parts[0].parse::<u32>().map_err(|_| "Error parsing u32")?;
        let end = parts[1].parse::<u32>().map_err(|_| "Error parsing u32")?;
        Ok(Assignment { start, end })
    }
}

fn get_assignment_pairs(lines: &[&str]) -> Vec<(Assignment, Assignment)> {
    lines.iter().map(|s| {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            panic!("Error parsing.");
        }
        (parts[0], parts[1])
    })
    .map(|(one,two)| (
        Assignment::try_from(one).unwrap(),
        Assignment::try_from(two).unwrap()
    )).collect()
}

fn part1(lines: &[&str]) -> u32 {
    get_assignment_pairs(lines).iter()
    .filter(|(one,two)| one.overlaps_with_fully(two))
    .count() as u32
}

fn part2(lines: &[&str]) -> u32 {
    get_assignment_pairs(lines).iter()
    .filter(|(one,two)| one.overlaps_with_at_all(two))
    .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;
    
    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TEST_INPUT.split('\n').filter(|s| !s.is_empty()).collect();
        assert_eq!(part1(&lines), 2);
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = TEST_INPUT.split('\n').filter(|s| !s.is_empty()).collect();
        assert_eq!(part2(&lines), 4);
    }
}
