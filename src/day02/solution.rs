#[derive(PartialEq, Clone, Copy, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn get_value(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }

    fn wins_to(&self) -> Self {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }
}

impl TryFrom<char> for Play {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Play::Rock),
            'B' | 'Y' => Ok(Play::Paper),
            'C' | 'Z' => Ok(Play::Scissors),
            _ => Err("Could not parse input"),
        }
    }
}

pub fn solution() {
    println!("Solution for day two part one: {}", part1());
    println!("Solution for day two part two: {}", part2());
}

fn part1() -> u32 {
    parse_input().iter().map(|(o,p)| calculate_points(o, p)).sum()
}

fn part2() -> u32 {
    parse_input().iter()
        .map(|(o,p)| (o, fix_play(o,p)))
        .map(|(o,p)| calculate_points(o, &p))
        .sum()
}

fn calculate_points(opponent: &Play, player: &Play) -> u32 {
    if opponent == player {
        player.get_value() + 3
    } else {
        let points = match (opponent, player) {
            (Play::Rock, Play::Paper) => 6,
            (Play::Rock, Play::Scissors) => 0,
            (Play::Paper, Play::Rock) => 0,
            (Play::Paper, Play::Scissors) => 6,
            (Play::Scissors, Play::Rock) =>  6,
            (Play::Scissors, Play::Paper) => 0,
            _ => 0
        };
        points + player.get_value()
    }
}

fn fix_play(opponent: &Play, player: &Play) -> Play {
    match player {
        Play::Rock => opponent.wins_to(),       // lose
        Play::Paper => *opponent,               // draw
        Play::Scissors => opponent.loses_to(),  // win
    }
}

fn parse_input() -> Vec<(Play, Play)> {
    let input: &str = include_str!("input.txt");

    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(parse_line)
        .map(|r| r.unwrap())
        .collect()
}

fn parse_line(line: &str) -> Result<(Play, Play), &'static str> {
    let chars: Vec<char> = line.replace(' ', "").trim().chars().collect();
    assert!(chars.len() == 2);
    Ok(
        ( Play::try_from(chars[0])?, Play::try_from(chars[1])?)
    )
}

#[cfg(test)]
mod test {
    use super::Play;

    fn get_data() -> Vec<(Play, Play)> {
        vec![
            (Play::Rock, Play::Paper),
            (Play::Paper, Play::Rock),
            (Play::Scissors, Play::Scissors),
        ]
    }

    #[test]
    fn part1() {
        assert_eq!(get_data().iter().map(|(o,p)| super::calculate_points(o, p)).sum::<u32>(), 15);
    }

    #[test]
    fn part2() {
        let fixed: Vec<(Play, Play)> = get_data().iter().map(|(o,p)| (*o, super::fix_play(o, p))).collect();
        println!("fixed = {:?}", fixed);
        assert_eq!(fixed.iter().map(|(o,p)| super::calculate_points(o, p)).sum::<u32>(), 12);
    }

}
