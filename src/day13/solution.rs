pub fn solution() {
    println!("Solution for day twelve part one = ???");
    println!("Solution for day twelve part two = ???");
}

enum Data {
  Int(i32),
  List(Vec<Data>),
}

impl Data {
  pub fn make_list(&self) -> Vec<Data> {
    use Data::*;
    match self {
      Int(x) => vec![Int(*x)],
      _ => self,
    }
  }
}

pub fn compare(lhs: &mut Packet, rhs: &mut Packet) -> bool {
  use Data::*;
  let left_data = lhs.pop();
  let right_data = rhs.pop();
  match (left_data, right_data) {
    (None, _) => true,
    (_, None) => false,
    (Some(Int(a)), Some(Int(b))) => {
      if a < b {
        true
      } else if a > b {
        false
      } else {
        compare(lhs, rhs)
      }
    },
    _ => false
  }
}

type Packet = Vec<Data>;