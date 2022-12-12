use priority_queue::PriorityQueue;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

const INPUT: &str = include_str!("input.txt");

const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

pub fn solution() {
    part1(INPUT);
    println!("Solution for day twelve part one = ???");
    println!("Solution for day twelve part two = ???");
}

fn get_height_map(input: &str) -> (HeightMap, usize, usize) {
    let heights = parse_input(input);
    let cols = input.chars().take_while(|c| *c != '\n').count();
    let rows = input.split('\n').count();
    let start_idx = input
        .replace("\n", "")
        .chars()
        .position(|c| c == 'S')
        .unwrap();
    let end_idx = input
        .replace("\n", "")
        .chars()
        .position(|c| c == 'E')
        .unwrap();
    let hmap = HeightMap::new(heights, rows, cols);
    (hmap, start_idx, end_idx)
}

fn heuristic(hmap: &HeightMap, a: usize, b: usize) -> i32 {
    let point_a = hmap.get_x_y_for_idx(a);
    let point_b = hmap.get_x_y_for_idx(b);
    (point_a.0 - point_b.0).abs() + (point_a.1 - point_b.1).abs()
    // hmap.heights[a] - hmap.heights[b]
}

fn part1(input: &str) {
    let (hmap, start_idx, end_idx) = get_height_map(input);
    println!("Start idx = {}, End idx = {}", start_idx, end_idx);

    // A* Search
    let mut frontier = PriorityQueue::new();
    let mut came_from: HashMap<usize, Option<usize>> = HashMap::new();
    let mut cost_so_far: HashMap<usize, i32> = HashMap::new();
    frontier.push(start_idx, 0);
    came_from.insert(start_idx, None);
    cost_so_far.insert(start_idx, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.0 == end_idx {
            println!("Found end index");
            break;
        }

        for next in hmap.get_node_for_index(current.0).neighbours {
            let new_cost = cost_so_far[&current.0] + 1;
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(&hmap, end_idx, next);
                frontier.push(next, -priority);
                came_from.insert(next, Some(current.0));
            }
        }
    }

    let mut current = end_idx;
    let mut path: Vec<usize> = Vec::new();
    while current != start_idx {
        path.push(current);
        if let Some(Some(c)) = came_from.get(&current) {
            current = *c
        } else {
            break;
        }
    }

    println!("{:?}", path);
    println!("{:?}", path.len());
    for (i,p) in hmap.heights.iter().enumerate() {
      if (i%hmap.cols) == 0 {
        println!();
      }
      if path.contains(&i) {
        print!("#") 
      } else {
        print!(".");
      }
    }
}

#[derive(Debug, PartialEq)]
struct HeightMap {
    heights: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl HeightMap {
    pub fn new(heights: Vec<i32>, rows: usize, cols: usize) -> Self {
        Self {
            heights,
            rows,
            cols,
        }
    }

    pub fn get_x_y_for_idx(&self, idx: usize) -> (i32, i32) {
        ((idx % self.cols) as i32, (idx / self.cols) as i32)
    }

    pub fn get_node_for_index(&self, idx: usize) -> Node {
        let row = idx / self.cols;
        let height = self.heights[idx];
        // let col = idx % self.cols;
        let row_range = (row * self.cols)..(row * self.cols + self.cols);
        let neighbour_indexes: Vec<usize> = vec![
            idx.checked_sub(self.cols), // index for element above
            idx.checked_add(self.cols), // index for element below
            idx.checked_sub(1).filter(|i| row_range.contains(i)), // index for element to the left
            idx.checked_add(1).filter(|i| row_range.contains(i)), // index for element to the right
        ]
        .iter()
        .flatten()
        .cloned()
        .filter(|i| {
            if let Some(neighbour_height) = self.heights.get(*i) {
                (height - neighbour_height) == -1
                    || (height - neighbour_height) >= 0
                    // || (height - neighbour_height) == 1
            } else {
                false
            }
        })
        .collect();

        Node {
            id: idx,
            height,
            neighbours: neighbour_indexes,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    pub id: usize,
    pub height: i32,
    pub neighbours: Vec<usize>,
}

impl Node {
    pub fn new(id: usize, height: i32, neighbours: Vec<usize>) -> Self {
        Self {
            id,
            height,
            neighbours,
        }
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .replace("\n", "")
        .chars()
        .map(char_to_height)
        .collect()
}

fn char_to_height(c: char) -> i32 {
    match c {
        'S' => 0,
        'E' => 26,
        l => (l as i32) - 'a' as i32,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_height_map() -> HeightMap {
        let heights = parse_input(TEST_INPUT);
        let cols = 8;
        let rows = 5;
        HeightMap::new(heights, rows, cols)
    }

    #[test]
    fn test_get_node_for_index() {
        let hmap = get_test_height_map();

        {
            let node = hmap.get_node_for_index(0);
            let expected_node = Node::new(0, 0, vec![8, 1]);
            assert_eq!(node, expected_node);
        }
        {
            let node = hmap.get_node_for_index(5);
            let expected_node = Node::new(5, 14, vec![4, 6]);
            assert_eq!(node, expected_node);
        }
    }

    #[test]
    fn test_get_x_y_for_idx() {
      let hmap = get_test_height_map();
      assert_eq!(hmap.get_x_y_for_idx(0), (0,0));
      assert_eq!(hmap.get_x_y_for_idx(7), (7,0));
      assert_eq!(hmap.get_x_y_for_idx(8), (0,1));
      assert_eq!(hmap.get_x_y_for_idx(10), (2,1));
    }
}
