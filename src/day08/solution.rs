const INPUT: &str = include_str!("input.txt");

use std::collections::HashSet;

pub fn solution() {
    let nums = parse_input(INPUT);
    let grid = TreeGrid::new_from_vec(nums, 99);
    let part1 = grid.count_high_trees();
    println!("Solution for day eight part one = {}", part1); 

    println!("Solution for day eight part two = {}", grid.find_most_scenic_tree());
}

fn parse_input(input: &str) -> Vec<u32> {
  input.trim().replace("\n", "").chars().map(|c| c.to_digit(10).unwrap()).collect()
}

struct TreeGrid {
    trees: Vec<(u32, usize)>,
    row_len: usize,
}

impl TreeGrid {
    pub fn new_from_vec(trees: Vec<u32>, row_len: usize) -> Self {
        assert_eq!(trees.len() % row_len as usize, 0);
        let trees_with_idx = trees.iter().enumerate().map(|(idx, n)| (*n, idx)).collect();
        Self {
            trees: trees_with_idx,
            row_len,
        }
    }

    pub fn get_row(&self, row: usize) -> Vec<(u32, usize)> {
        let start_idx = (self.row_len as usize) * row;
        self.trees
            .iter()
            .skip(start_idx)
            .take(self.row_len as usize)
            .cloned()
            .collect()
    }

    pub fn get_column(&self, col: usize) -> Vec<(u32, usize)> {
        (0..self.row_len)
            .map(|n| (n * self.row_len) as usize + col)
            .map(|idx| self.trees[idx])
            .collect()
    }

    fn take_while_cnd(last: &mut u32, n: &u32) -> bool {
      if n > last {
        *last = *n;
        true
      } else {
        false
      }
    }

    pub fn find_most_scenic_tree(&self) -> u32 {
      let mut scores: Vec<u32> = (0..self.trees.len()).map(|idx| self.scenic_score_for_tree(idx)).collect();
      scores.sort();
      scores[scores.len()-1]
    }

    pub fn scenic_score_for_tree(&self, idx: usize) -> u32 {
      let tree_size: u32 = self.trees[idx].0.clone();
      let row = idx / self.row_len;
      let column = idx % self.row_len;
      let left: Vec<(u32, usize)> = self.get_row(row).iter().cloned().take(column).rev().collect();
      let right: Vec<(u32, usize)> = self.get_row(row).iter().cloned().skip(column+1).collect();
      let above: Vec<(u32, usize)> = self.get_column(column).iter().cloned().take(row).rev().collect();
      let below: Vec<(u32, usize)> = self.get_column(column).iter().cloned().skip(row+1).collect();
      let a = Self::find_highest_part2(&left, tree_size);
      let b = Self::find_highest_part2(&right, tree_size);
      let c = Self::find_highest_part2(&above, tree_size);
      let d = Self::find_highest_part2(&below, tree_size);
      a * b * c * d
    }

    pub fn find_highest_part2(nums: &Vec<(u32, usize)>, size: u32) -> u32 {
      if nums.is_empty() {
        return 0;
      }
      // skip first and last element
      let smaller_trees: Vec<(u32, usize)> = nums.iter()
          .take_while(|(n, _)| *n < size)
          .cloned().collect();
      if (smaller_trees.len() == nums.len()) {
        smaller_trees.len() as u32
      } else {
        (smaller_trees.len() + 1) as u32
      }
    }

    pub fn find_highest(nums: &Vec<(u32, usize)>, row_len: usize) -> Vec<(u32, usize)> {
      let mut last_size: u32 = nums[0].0.clone();
      // skip first and last element
      nums.iter().skip(1).take(row_len as usize - 2)
          .filter(|(n, _)| Self::take_while_cnd(&mut last_size, n))
          .cloned()
          .collect()
    }

    pub fn find_highest_both_dirs(nums: &Vec<(u32, usize)>, row_len: usize) -> Vec<(u32, usize)> {
      let mut x = Self::find_highest(nums, row_len);
      let rev: Vec<(u32, usize)> = nums.iter().cloned().rev().collect();
      let mut y = Self::find_highest(&rev, row_len);
      x.append(&mut y);
      x
    }

    pub fn count_high_trees(&self) -> u32 {
        let mut count: HashSet<usize> = HashSet::new();
        for row_idx in 1..(self.row_len - 1) as usize {
          let row: Vec<(u32,usize)> = self.get_row(row_idx);
          Self::find_highest_both_dirs(&row, self.row_len as usize).iter().for_each(|(_, id)| {count.insert(*id);})
        }

        for col_idx in 1..(self.row_len-1) as usize {
          let col= self.get_column(col_idx);
          Self::find_highest_both_dirs(&col, self.row_len as usize).iter().for_each(|(_, id)| {count.insert(*id);})
        }
        (count.len() + (self.row_len*4) - 4) as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_create_tree() {
        let nums: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];
        let tree_grid = TreeGrid::new_from_vec(nums, 5);
        assert_eq!(tree_grid.get_row(0).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![3, 0, 3, 7, 3]);
        assert_eq!(tree_grid.get_row(1).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![2, 5, 5, 1, 2]);
        assert_eq!(tree_grid.get_row(2).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![6, 5, 3, 3, 2]);
        assert_eq!(tree_grid.get_row(3).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![3, 3, 5, 4, 9]);
        assert_eq!(tree_grid.get_row(4).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![3, 5, 3, 9, 0]);
    }

    #[test]
    fn test_tree_grid_get_column() {
        let nums: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];
        let tree_grid = TreeGrid::new_from_vec(nums, 5);
        assert_eq!(tree_grid.get_column(0).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![3, 2, 6, 3, 3]);
        assert_eq!(tree_grid.get_column(1).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![0, 5, 5, 3, 5]);
        assert_eq!(tree_grid.get_column(2).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![3, 5, 3, 5, 3]);
        assert_eq!(tree_grid.get_column(3).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![7, 1, 3, 4, 9]);
        assert_eq!(tree_grid.get_column(4).iter().map(|(n, _)| *n).collect::<Vec<u32>>(), vec![3, 2, 2, 9, 0]);
    }

    #[test]
    fn test_count_high_trees() {
        let nums: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];
        let tree_grid = TreeGrid::new_from_vec(nums, 5);
        assert_eq!(tree_grid.count_high_trees(), 21);
    }

    #[test]
    fn test_find_highest() {
        let nums: Vec<(u32, usize)> = vec![(2, 0), (5, 1), (5, 2), (1, 3), (2, 4)];
        assert_eq!(
          TreeGrid::find_highest(&nums, 5),
          vec![(5,1)]
        );
    }

    #[test]
    fn test_find_highest_both_dirs() {
        let nums: Vec<(u32, usize)> = vec![(2, 0), (5, 1), (5, 2), (1, 3), (2, 4)];
        assert_eq!(
          TreeGrid::find_highest_both_dirs(&nums, 5),
          vec![(5,1), (5, 2)]
        );
    }

    #[test]
    fn test_find_scenic_values() {
        let nums: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];
        let tree_grid = TreeGrid::new_from_vec(nums, 5);
        let scores: Vec<u32> = (0..25).map(|idx| tree_grid.scenic_score_for_tree(idx)).collect();
        println!("scores = {:?}", scores);
        assert!(false);
    }
}
