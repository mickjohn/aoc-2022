
pub fn solution() {
    println!("2021 Day 12 Part 1 = ???");
}

struct Node<'a> {
    pub name: &'a str,
    pub neighbours: Vec<Node<'a>>,
    pub small_neighbours: Vec<Node<'a>>,
    pub big_neighbours: Vec<Node<'a>>,
    pub dead_end: bool,
}

impl<'a> Node<'a> {

    pub fn new(name: &str, nodes: Vec<Node>) {
        Self {
            name: name,
            neighbours: nodes.clone(),
            small_neighbours: nodes.iter().cloned(),

        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tree() {
        let n = 
    }
}