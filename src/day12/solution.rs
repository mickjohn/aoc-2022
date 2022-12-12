use priority_queue::PriorityQueue;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

pub fn solution() {
    println!("Solution for day twelve part one = {}", part1(INPUT));
    println!("Solution for day twelve part two = {}", part2(INPUT));
}

fn get_height_map(input: &str) -> HeightMap {
    let heights = parse_input(input);
    let cols = input.chars().take_while(|c| *c != '\n').count();
    let rows = input.split('\n').count();
    HeightMap::new(heights, rows, cols)
}

fn part2(input: &str) -> usize {
    let chars: Vec<char> = input.replace('\n', "").chars().collect();
    let low_points: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'a')
        .map(|(i, _)| i)
        .collect();
    let end_idx = chars.iter().position(|c| *c == 'E').unwrap();
    let hmap = get_height_map(input);

    let mut distances: Vec<usize> = low_points
        .iter()
        .map(|start_idx| a_star_algo(&hmap, *start_idx, end_idx))
        .filter(|path_len| *path_len > 1)
        .collect();
    distances.sort();
    distances[0]
}

fn part1(input: &str) -> usize {
    let chars: Vec<char> = input.replace('\n', "").chars().collect();
    let start_idx = chars.iter().position(|c| *c == 'S').unwrap();
    let end_idx = chars.iter().position(|c| *c == 'E').unwrap();
    let hmap = get_height_map(input);
    a_star_algo(&hmap, start_idx, end_idx)
}

fn a_star_algo(hmap: &HeightMap, start_idx: usize, end_idx: usize) -> usize {
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
            break;
        }

        for next in hmap.get_node_for_index(current.0).neighbours {
            let new_cost = cost_so_far[&current.0] + 1;
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + hmap.manhattan_distance(end_idx, next);
                frontier.push(next, -priority);
                came_from.insert(next, Some(current.0));
            }
        }
    }

    let path = get_path(came_from, start_idx, end_idx);
    // pretty_print_path(&hmap, &path);
    path.len()
}

fn pretty_print_path(hmap: &HeightMap, path: &[usize]) {
    for (i, _) in hmap.heights.iter().enumerate() {
        if (i % hmap.cols) == 0 {
            println!();
        }
        if path.contains(&i) {
            print!("#")
        } else {
            print!(".");
        }
    }
}

fn get_path(paths: HashMap<usize, Option<usize>>, start: usize, goal: usize) -> Vec<usize> {
    let mut current = goal;
    let mut path: Vec<usize> = Vec::new();
    while current != start {
        path.push(current);
        if let Some(Some(c)) = paths.get(&current) {
            current = *c
        } else {
            break;
        }
    }
    path
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

    pub fn get_point_for_idx(&self, idx: usize) -> (i32, i32) {
        ((idx % self.cols) as i32, (idx / self.cols) as i32)
    }

    pub fn manhattan_distance(&self, from: usize, to: usize) -> i32 {
        let point_a = self.get_point_for_idx(from);
        let point_b = self.get_point_for_idx(to);
        (point_a.0 - point_b.0).abs() + (point_a.1 - point_b.1).abs()
    }

    pub fn get_node_for_index(&self, idx: usize) -> Node {
        let row = idx / self.cols;
        let height = self.heights[idx];
        let row_range = (row * self.cols)..(row * self.cols + self.cols);

        // Get the indexes of the four neighbours of idx
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
                (height - neighbour_height) == -1 || (height - neighbour_height) >= 0
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
        .replace('\n', "")
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
    fn test_get_point_for_idx() {
        let hmap = get_test_height_map();
        assert_eq!(hmap.get_point_for_idx(0), (0, 0));
        assert_eq!(hmap.get_point_for_idx(7), (7, 0));
        assert_eq!(hmap.get_point_for_idx(8), (0, 1));
        assert_eq!(hmap.get_point_for_idx(10), (2, 1));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 391);
    }
}
