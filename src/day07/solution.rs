use std::cell::RefCell;
use nom::{IResult, bytes::complete::{tag, take_while}, sequence::tuple, character::complete::{alpha1, char}};
use nom::branch::alt;
use nom::combinator::map_res;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
use std::rc::Rc;


const INPUT: &str = include_str!("input.txt");

pub fn solution() {
  // Part 1
  let (_, lines) = parse_input(INPUT).unwrap();
  let fs = build_fs(lines);
  let mut dirs = Vec::new();
  gather_dirs(&fs.borrow(), &mut dirs);
  let part1_answer: u32 = dirs.iter().cloned().filter(|n| *n<=100000).sum();
  println!("Solution for day seven part one = {}", part1_answer); 


  // Part 2
  let space_used = fs.borrow().get_size();
  let unused_space = 70000000 - space_used;
  let target = 30000000;
  let mut candidates_for_deletion: Vec<u32> = dirs.iter().cloned().filter(|n| (unused_space+n) >= target).collect();
  candidates_for_deletion.sort();
  let part2_answer = candidates_for_deletion[0];
  println!("Solution for day seven part two = {}", part2_answer); 
}

// A struct to hold a line of input 
#[derive(PartialEq, Debug)]
enum Line {
  Cd(String),
  Ls,
  FileLine(File),
  DirLine(Dir),
}

#[derive(PartialEq, Debug)]
struct File {
  pub name: String,
  pub size: u32,
}

// The Dir is a tree like stucture. It uses referenced counted variables, with
// RefCells inside, which allow interior mutability without needing to declare a var
// as 'mut'. This is moving compile time safety checks to runtime.
#[derive(PartialEq, Debug)]
struct Dir {
  pub name: String,
  pub files: Vec<Rc<RefCell<File>>>,
  pub dirs: Vec<Rc<RefCell<Dir>>>,
  pub parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
  // recursively get size
  pub fn get_size(&self) -> u32 {
    let files = self.files.iter().map(|f| f.borrow().size).sum();
    if self.dirs.is_empty() {
      files
    } else {
      self.dirs.iter().map(|d| d.borrow().get_size()).sum::<u32>() + files
    }
  }
}

// The allowed characters for a filename or a directory
fn is_valid_path_char(c: char) -> bool {
  c.is_alphabetic() || c == '.' || c == '/'
}

// Parse the 'cd' command
fn parse_cd(input: &str) -> IResult<&str, Line> {
  tuple((
    tag("$ cd "),
    take_while(is_valid_path_char)
  ))(input).map(|(i,(_, o))| {
    (i, Line::Cd(o.to_owned()))
  })
}

// Parse the ls command
fn parse_ls(input: &str) -> IResult<&str, Line> {
  tag("$ ls")(input).map(|(i,_o)| {
    (i, Line::Ls)
  })
}

// Parse the dir entry
fn parse_dir(input: &str) -> IResult<&str, Line> {
  tuple((
    tag("dir "),
    alpha1
  ))(input).map(|(i,(_, o))| {
    let d = Dir { name: o.to_string(), files: Vec::new(), dirs: Vec::new(), parent: None };
    (i, Line::DirLine(d))
  })
}

// Parse a decimal number
fn decimal(input: &str) -> IResult<&str, u32> {
  map_res(digit1, str::parse)(input)
}

// Parse the file
fn parse_file(input: &str) -> IResult<&str, Line> {
  tuple((
    decimal,
    tag(" "),
    take_while(is_valid_path_char)
  ))(input).map(|(i,(size, _, name))| {
    let f = File { name: name.to_string(), size };
    (i, Line::FileLine(f))
  })
}

// Parse a line from today's challange
fn parse_line(input: &str) -> IResult<&str, Line> {
  alt((
    parse_cd,
    parse_ls,
    parse_dir,
    parse_file
  ))(input)
}

// Parse all of today's input
fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
  separated_list1(char('\n'), parse_line)(input)
}

// Traverse the 'filesystem' and gather the size of all dirs as a Vec
fn gather_dirs(dir: &Dir, dirs: &mut Vec<u32>) {
  for d in &dir.dirs {
    dirs.push(d.borrow().get_size());
    gather_dirs(&d.borrow(), dirs);
  }
}

fn build_fs(lines: Vec<Line>) -> Rc<RefCell<Dir>> {
  let root = Rc::new(RefCell::new(Dir { name: "/".to_string(), files: Vec::new(), dirs: Vec::new(), parent: None }));
  let mut current_dir: Rc<RefCell<Dir>> = Rc::clone(&root);

  for line in lines {
    match line {
      Line::Cd(dir) if dir == "/" => (),
      Line::Cd(dir) if dir == ".." => {
        let parent = Rc::clone(current_dir.borrow().parent.as_ref().unwrap());
        current_dir = parent;
      }
      Line::Cd(dir) => {
        let child = Rc::clone(current_dir.borrow().dirs.iter().find(|d| d.borrow().name == dir).as_ref().unwrap());
        current_dir = child;
      },
      Line::FileLine(f) => {
        current_dir.borrow_mut().files.push(Rc::new(RefCell::new(f)))
      },
      Line::DirLine(mut d) => {
        d.parent = Some(Rc::clone(&current_dir));
        current_dir.borrow_mut().dirs.push(Rc::new(RefCell::new(d)))
      },
      _ => (),
    }
  }
  root
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_cd() {
    let table = vec![
      ("$ cd /", "", Line::Cd("/".to_string())),
      ("$ cd fdshjfsd\n", "\n", Line::Cd("fdshjfsd".to_string())),
      ("$ cd fdshjfsd.txt", "", Line::Cd("fdshjfsd.txt".to_string())),
    ];

    for (input, remaining, expected) in table {
      let (r, cd) = parse_cd(input).unwrap();
      assert_eq!(r, remaining);
      assert_eq!(expected, cd);
    }
  }

  #[test]
  fn test_parse_ls() {
    let table = vec![
      ("$ ls", "", Line::Ls),
      ("$ ls\n", "\n", Line::Ls),
    ];

    for (input, remaining, expected) in table {
      let (r, ls) = parse_ls(input).unwrap();
      assert_eq!(r, remaining);
      assert_eq!(expected, ls);
    }
  }


  #[test]
  fn test_parse_dir() {
    let table = vec![
      ("dir abcd", "", Line::DirLine(Dir{name: "abcd".to_string(), files: Vec::new(), dirs: Vec::new(), parent: None})),
      ("dir abcd\n", "\n", Line::DirLine(Dir{name: "abcd".to_string(), files: Vec::new(), dirs: Vec::new(), parent: None})),
    ];

    for (input, remaining, expected) in table {
      let (r, dir) = parse_dir(input).unwrap();
      assert_eq!(r, remaining);
      assert_eq!(expected, dir);
    }
  }
  
  #[test]
  fn test_parse_file() {
    let table = vec![
      ("100100 file.txt", "", Line::FileLine(File{name: "file.txt".to_string(), size: 100100})),
      ("9 file.txt\n", "\n", Line::FileLine(File{name: "file.txt".to_string(), size: 9})),
    ];

    for (input, remaining, expected) in table {
      let (r, file) = parse_file(input).unwrap();
      assert_eq!(r, remaining);
      assert_eq!(expected, file);
    }
  }

  #[test]
  fn test_parse_input() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f";

    let expected = vec![
      Line::Cd("/".to_string()),
      Line::Ls,
      Line::DirLine(Dir{ name: "a".to_string(), files: Vec::new(), dirs: Vec::new(), parent: None }),
      Line::FileLine(File { name: "b.txt".to_string(), size: 14848514 }),
      Line::FileLine(File { name: "c.dat".to_string(), size: 8504156 }),
      Line::DirLine(Dir{ name: "d".to_string(), files: Vec::new(), dirs: Vec::new(), parent: None }),
      Line::Cd("a".to_string()),
      Line::Ls,
      Line::DirLine(Dir{ name: "e".to_string(), files: Vec::new(), dirs: Vec::new(), parent: None }),
      Line::FileLine(File { name: "f".to_string(), size: 29116 }),
    ];

    let (_, lines) = parse_input(input).unwrap();
    assert_eq!(lines, expected);
  }



}