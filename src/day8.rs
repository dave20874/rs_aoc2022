use std::collections::HashSet;
use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day8 {
    grid: Vec<Vec<usize>>,
}

impl Day8 {
    pub fn load(filename: &str) -> Day8 {
        let mut grid: Vec<Vec<usize>> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let mut row: Vec<usize> = Vec::new();
            for c in l.trim().chars() {
                let height = c.to_digit(10).unwrap();
                row.push(height as usize);
            }
            grid.push(row);
        }

        Day8 { grid }
    }

    fn find_visibles(&self) -> HashSet<(usize, usize)> {
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        let mut visibles = HashSet::new();

        // Process all rows
        for y in 0..rows {
            // left edge is visible
            visibles.insert((y, 0));
            // println!("left edge: ({}, {})", y, 0);
            // right edge is visible
            visibles.insert((y, cols - 1));
            // println!("right edge: ({}, {})", y, cols-1);

            // Look from left to right on each row.
            // All increases represent visible trees.
            let mut visible_height = self.grid[y][0];
            for x in 1 .. self.grid[0].len() - 1 {
                if self.grid[y][x] > visible_height {
                    visibles.insert((y, x));
                    // println!("Visible from left: ({}, {})", y, x);
                    visible_height = self.grid[y][x];
                }
            }

            // Look from right to left on each row.  First element and all increases
            // All increases represent visible trees.
            let mut visible_height = self.grid[y][cols - 1];
            for x in (1 .. cols-1).rev() {
                if self.grid[y][x] > visible_height {
                    visibles.insert((y, x));
                    // println!("Visible from right: ({}, {})", y, x);
                    visible_height = self.grid[y][x];
                }
            }
        }

        // Process all cols
        for x in 0..cols {
            // top edge is visible
            visibles.insert((0, x));
            // println!("Top edge: ({}, {})", 0, x);
            // bottom edge is visible
            visibles.insert((rows-1, x));
            // println!("Bottom edge: ({}, {})", rows-1, x);

            // Look from top to bottom on each col.
            // All increases represent visible trees.
            let mut visible_height = self.grid[0][x];
            for y in 1..rows-1 {
                if self.grid[y][x] > visible_height {
                    visibles.insert((y, x));
                    // println!("Visible from above: ({}, {})", y, x);
                    visible_height = self.grid[y][x];
                }
            }

            // Look from right to left on each row.  First element and all increases
            // All increases represent visible trees.
            let mut visible_height = self.grid[rows-1][x];
            for y in (1 .. rows-1).rev() {
                if self.grid[y][x] > visible_height {
                    visibles.insert((y, x));
                    // println!("Visible from below: ({}, {})", y, x);
                    visible_height = self.grid[y][x];
                }
            }
        }

        visibles
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let max = self.grid[row][col];
        let cols = self.grid[0].len();
        let rows = self.grid.len();

        let mut up_dist = 0;
        let mut down_dist = 0;
        let mut left_dist = 0;
        let mut right_dist = 0;

        // check up
        for y in (0..row).rev() {
            up_dist += 1;
            if self.grid[y][col] >= max {
                break;
            }
        }
        // check down
        for y in row+1..rows {
            down_dist += 1;
            if self.grid[y][col] >= max {
                break;
            }
        }

        // check left
        for x in (0..col).rev() {
            left_dist += 1;
            if self.grid[row][x] >= max {
                break;
            }
        }
        // check right
        for x in col+1..cols {
            right_dist += 1;
            if self.grid[row][x] >= max {
                break;
            }
        }

        // println!("{} * {} * {} * {}", up_dist, left_dist, right_dist, down_dist);
        up_dist * down_dist * left_dist * right_dist
    }

    fn highest_scenic_score(&self) -> usize {
        let cols = self.grid[0].len();
        let rows = self.grid.len();
        let mut max = 0;

        for y in 0..rows {
            for x in 0..cols {
                let score = self.scenic_score(y, x);
                if score > max {
                    max = score;
                }
            }
        }

        max
    }
}

impl Day for Day8 {
    fn part1(&self) -> Answer {
        let visibles = self.find_visibles();

        Answer::Number(visibles.len())
    }

    fn part2(&self) -> Answer {
        Answer::Number(self.highest_scenic_score())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day8::load("examples/day8_example1.txt");
        assert_eq!(d.grid.len(), 5);
        assert_eq!(d.grid[0].len(), 5);
        assert_eq!(d.grid[0][3], 7);
    }

    #[test]
    fn test_find_visibles() {
        let d = Day8::load("examples/day8_example1.txt");
        assert_eq!(d.find_visibles().len(), 21);
    }

    #[test]
    fn test_scenic_score() {
        let d = Day8::load("examples/day8_example1.txt");
        assert_eq!(d.scenic_score(1, 2), 4);
        assert_eq!(d.scenic_score(3, 2), 8);
    }

    #[test]
    fn test_highest_scenic_score() {
        let d = Day8::load("examples/day8_example1.txt");
        assert_eq!(d.highest_scenic_score(), 8);
    }
}
