use std::fmt;

pub fn solution() {
    println!("Solution for day twelve part one = ???");
    println!("Solution for day twelve part two = ???");
}

#[derive(Debug, Clone)]
pub enum Data {
    Int(i32),
    List(Vec<Data>),
}

impl Data {
    pub fn make_list(&self) -> Data {
        use Data::*;
        match self {
            Int(x) => List(vec![Int(*x)]),
            list => self.clone(),
        }
    }
}

// impl fmt::Display for Data {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       match self {
//         Int(x) => write!(f, "{}", x),
//         List(xs)
//       }
//         write!(f, "{}", self.0)
//     }
// }

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
        }
        //TODO compare lists
        _ => false,
    }
}

pub type Packet = Vec<Data>;
