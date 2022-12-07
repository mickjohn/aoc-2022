use nom::{IResult, bytes::complete::tag, sequence::tuple, multi::many1, character::complete::{anychar, alpha1}};



pub fn solution() {
  println!("Solution for day seven part one = ???");
  println!("Solution for day seven part two = ???");
}

#[derive(PartialEq, Debug)]
struct Cd(String);

#[derive(PartialEq, Debug)]
struct Dir(String);

#[derive(PartialEq, Debug)]
struct File(String, u32);

#[derive(PartialEq)]
struct Ls;

fn parse_cd(input: &str) -> IResult<&str, Cd> {
  tuple((
    tag("$ cd "),
    many1(anychar)
  ))(input).map(|(i,(_, o))| {
    let cd: Cd = Cd(o.into_iter().collect());
    (i, cd)
  })
}

fn parse_ls(input: &str) -> IResult<&str, Ls> {
  tag("$ ls")(input).map(|(i,_o)| {
    (i, Ls)
  })
}

fn parse_dir(input: &str) -> IResult<&str, Entry> {
  tuple((
    tag("dir "),
    alpha1
  ))(input).map(|(i,(_, o))| {
    let e = Entry::Dir { name: o.to_string(), contents: Vec::new() };
    (i, e)
  })
}

enum Entry {
  File{name: String, size: u32},
  Dir{name: String, contents: Vec<Entry>},
  Parent,
}

impl Entry {
  pub fn get_size(&self) -> u32 {
    match &self {
      Entry::File{size, ..} => *size,
      Entry::Dir{contents, ..} => {
          contents.iter().fold(0, |acc, e| acc + e.get_size())
      },
      _ => 0,
    }
  }
}

// just used for debugging
fn visit_dirs(entry: &Entry) {
  match entry {
    Entry::File { .. } => (),
    Entry::Dir {ref name, ref contents } => {
      println!("{} size = {}", name, entry.get_size());
      contents.iter().for_each(visit_dirs);
    },
    _ => (),
  };
}

// struct Root<'a> {
//   contents: Vec<Entry>,
//   current_node: Option<&'a mut Entry>,
// }

// impl<'a> Root<'a> {
//   pub fn new() -> Self {
//     Self {
//       contents: Vec::new(),
//       current_node: None,
//     }
//   }

//   pub fn add_entry(&mut self, mut e: Entry) {
//     // if let Some(current_node) = self.current_node.as_mut() {
//     //   match current_node {
//     //     Entry::Dir { contents , ..} => contents.push(e),
//     //     _ => panic!("Can only add entries to 'Dir'"),
//     //   }
//     // }
//     self.contents.push(e);
//   }

//   pub fn cd(&mut self, path: &str) {
//     for entry in self.contents {
//       match entry {
//         Entry::File { name, .. } if name == "" => {

//         }
//         _ => (),
//       };
//     }
//     // Can only go one dir at a time
//     assert!(!path.contains("/"));
//     todo!()
//   }
// }

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_fs() {
    let fs = Entry::Dir { name: "/".to_string(), contents: vec![
      Entry::File { name: "a".to_string(), size: 500 },
      Entry::File { name: "b".to_string(), size: 500 },
      Entry::File { name: "c".to_string(), size: 500 },
      Entry::Dir { name: "xyz".to_string(), contents: vec![
        Entry::File {name: "q".to_string(), size: 1},
      ]},
    ]};
    assert_eq!(fs.get_size(), 1501);
  }

  #[test]
  fn test_parse_cd() {
    let table = vec![
      ("$ cd /", "", Cd("/".to_string())),
      ("$ cd fdshjfsd", "", Cd("fdshjfsd".to_string())),
      ("$ cd fdshjfsd.txt", "", Cd("fdshjfsd.txt".to_string())),
    ];

    for (input, remaining, expected) in table {
      let (r, cd) = parse_cd(input).unwrap();
      assert!(r.is_empty());
      assert_eq!(expected, cd);
    }

  }

}