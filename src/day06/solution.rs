use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

pub fn solution() {
  println!("Solution for day six part one = {}", find_marker(INPUT.trim(), 4));
  println!("Solution for day six part two = {}", find_marker(INPUT.trim(), 14));
}

fn find_marker(s: &str, marker_len: u32) -> u32 {
  let letters: Vec<char> = s.chars().collect();
  let mut char_count = 0;

  for window in letters.as_slice().windows(marker_len as usize) {
    let temp_set: HashSet<&char> = HashSet::from_iter(window.iter());
    if temp_set.len() == (marker_len as usize) {
      char_count += marker_len;
      break;
    } else {
      char_count += 1;
    }
  }
  char_count
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_part1() {
    let test_strings = vec![
      ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
      ("nppdvjthqldpwncqszvftbrmjlhg", 6),
      ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
      ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)
    ];

    for (test_str, expected) in test_strings {
      assert_eq!(find_marker(test_str, 4), expected);
    }
  }

  #[test]
  fn test_part2() {
    let test_strings = vec![
      ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
      ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
      ("nppdvjthqldpwncqszvftbrmjlhg", 23),
      ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
      ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)
    ];

    for (test_str, expected) in test_strings {
      assert_eq!(find_marker(test_str, 14), expected);
    }
  }

}