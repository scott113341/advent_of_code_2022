use crate::constant_iterator::ConstantIterator;
use std::collections::HashSet;
use Direction::*;

pub type Height = i8;
pub type Trees = Vec<Height>;

pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Debug)]
pub struct TreeGrid {
    trees: Trees,
    width: usize,
    height: usize,
}

impl TreeGrid {
    pub fn new(trees: Trees, width: usize, height: usize) -> TreeGrid {
        assert_eq!(trees.len(), width * height);
        TreeGrid {
            trees,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tree_idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    // Number of trees visible from outside the grid
    pub fn visible_count(&self) -> usize {
        let mut visible_trees = HashSet::new();
        visible_trees.extend(self.visible_from_side(Top));
        visible_trees.extend(self.visible_from_side(Bottom));
        visible_trees.extend(self.visible_from_side(Left));
        visible_trees.extend(self.visible_from_side(Right));
        visible_trees.len()
    }

    // Multiply the number of trees visible from each of the 4 directions
    pub fn scenic_score(&self, row: usize, col: usize) -> usize {
        let tree_idx = self.tree_idx(row, col);
        let tree_height = self.trees[tree_idx];

        // Left
        let mut count_left = 0;
        for c in 1..=col {
            count_left += 1;
            let h = self.trees[self.tree_idx(row, col - c)];
            if h >= tree_height {
                break;
            }
        }

        // Right
        let mut count_right = 0;
        for c in 1..(self.width - col) {
            count_right += 1;
            let h = self.trees[self.tree_idx(row, col + c)];
            if h >= tree_height {
                break;
            }
        }

        // Up
        let mut count_up = 0;
        for r in 1..=row {
            count_up += 1;
            let h = self.trees[self.tree_idx(row - r, col)];
            if h >= tree_height {
                break;
            }
        }

        // Down
        let mut count_down = 0;
        for r in 1..(self.height - row) {
            count_down += 1;
            let h = self.trees[self.tree_idx(row + r, col)];
            if h >= tree_height {
                break;
            }
        }

        count_left * count_right * count_up * count_down
    }

    fn visible_from_side(&self, from_side: Direction) -> HashSet<usize> {
        let mut visible_tree_idxs = HashSet::new();

        match from_side {
            Top => {
                for col in 0..self.width {
                    let rows_col = (0..self.height).zip(ConstantIterator::new(col));
                    visible_tree_idxs.extend(self.visible_along_row_col(rows_col));
                }
            }
            Bottom => {
                for col in 0..self.width {
                    let rows_col = (0..self.height).rev().zip(ConstantIterator::new(col));
                    visible_tree_idxs.extend(self.visible_along_row_col(rows_col));
                }
            }
            Left => {
                for row in 0..self.height {
                    let row_cols = ConstantIterator::new(row).zip(0..self.width);
                    visible_tree_idxs.extend(self.visible_along_row_col(row_cols));
                }
            }
            Right => {
                for row in 0..self.height {
                    let row_cols = ConstantIterator::new(row).zip((0..self.width).rev());
                    visible_tree_idxs.extend(self.visible_along_row_col(row_cols));
                }
            }
        }

        visible_tree_idxs
    }

    fn visible_along_row_col(&self, iter: impl Iterator<Item = (usize, usize)>) -> HashSet<usize> {
        let mut visible_idxs = HashSet::new();
        let mut highest_so_far = -1;

        for (row, col) in iter {
            let tree_idx = self.tree_idx(row, col);
            let height = self.trees[tree_idx];
            let is_new_highest = height > highest_so_far;
            if is_new_highest {
                visible_idxs.insert(tree_idx);
                highest_so_far = height;
            }
        }

        visible_idxs
    }
}
