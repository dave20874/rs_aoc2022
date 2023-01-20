use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day12 {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<usize>>,
}

impl Day12 {
    pub fn load(filename: &str) -> Day12 {
        let mut map: Vec<Vec<usize>> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut col_no = 0;
        let mut row_no = 0;

        for line in reader.lines() {
            let l = &line.unwrap();
            let mut row: Vec<usize> = Vec::new();
            for c in l.trim().chars() {
                let height = match c {
                    'S' => {
                        start = (row_no, col_no);
                        1
                    } 
                    'E' => {
                        end = (row_no, col_no);
                        26
                    }
                    'a'..='z' => {
                        (c as usize) - ('a' as usize) + 1
                    }
                    _ => 0
                };
                row.push(height);
                col_no += 1;
            }
            map.push(row);
            row_no += 1;
            col_no = 0;
        }

        Day12 { start, end, map }
    }

    fn path_len(&self)  -> usize {
        // make a distance matrix initialized with zeros
        let mut distance: Vec<Vec<usize>> = Vec::new();
        for row in &self.map {
            let mut new_row: Vec<usize> = Vec::new();
            for _ in row {
                new_row.push(1000000);
            }
            distance.push(new_row);
        }

        // set distance to start location to zero
        distance[self.start.0][self.start.1] = 0;

        // now do a bunch of passes on the table updating all unvisited cells bordering visited ones
        let mut steps = 0;
        let mut solved = false;
        while !solved {
            steps += 1;
            for row in 0..self.map.len() {
                for col in 0..self.map[row].len() {
                    let h = self.map[row][col];

                    if distance[row][col] == 1000000 {
                        if ((row > 0) && (distance[row-1][col] == steps-1) && (self.map[row-1][col] >= h-1)) ||
                           ((row < self.map.len()-1) && (distance[row+1][col] == steps-1) && (self.map[row+1][col] >= h-1)) ||
                           ((col > 0) && (distance[row][col-1] == steps-1) && (self.map[row][col-1] >= h-1)) ||
                           ((col < self.map[row].len()-1) && (distance[row][col+1] == steps-1) && (self.map[row][col+1] >= h-1)) { 
                            // println!("Setting {row},{col} to {steps}");
                            distance[row][col] = steps;
                        }
                    }
                }
            }

            if distance[self.end.0][self.end.1] == steps {
                // println!("Reached end at {}, {}", self.end.0, self.end.1);
                solved = true;
            }
        }

        steps
    }

    fn rev_path_len(&self)  -> usize {
        // make a distance matrix initialized with zeros
        let mut distance: Vec<Vec<usize>> = Vec::new();
        for row in &self.map {
            let mut new_row: Vec<usize> = Vec::new();
            for _ in row {
                new_row.push(1000000);
            }
            distance.push(new_row);
        }

        // set distance to end location to zero
        distance[self.end.0][self.end.1] = 0;

        // now do a bunch of passes on the table updating all unvisited cells bordering visited ones
        let mut steps = 0;
        let mut solved = false;
        while !solved {
            steps += 1;
            for row in 0..self.map.len() {
                for col in 0..self.map[row].len() {
                    let h = self.map[row][col];

                    if distance[row][col] == 1000000 {
                        if ((row > 0) && (distance[row-1][col] == steps-1) && (self.map[row-1][col] <= h+1)) ||
                           ((row < self.map.len()-1) && (distance[row+1][col] == steps-1) && (self.map[row+1][col] <= h+1)) ||
                           ((col > 0) && (distance[row][col-1] == steps-1) && (self.map[row][col-1] <= h+1)) ||
                           ((col < self.map[row].len()-1) && (distance[row][col+1] == steps-1) && (self.map[row][col+1] <= h+1)) { 
                            // println!("Setting {row},{col} to {steps}");
                            distance[row][col] = steps;

                            // check for reached level 1
                            if self.map[row][col] == 1 {
                                solved = true;
                            }
                        }
                    }
                }
            }
        }

        steps
    }
}

impl Day for Day12 {
    fn part1(&self) -> Answer {
        Answer::Number(self.path_len())
    }

    fn part2(&self) -> Answer {
        Answer::Number(self.rev_path_len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day12::load("examples/day12_example1.txt");
        assert_eq!(d.map.len(), 5);
        assert_eq!(d.map[0].len(), 8);
        assert_eq!(d.start, (0, 0));
        assert_eq!(d.end, (2, 5));
    }

    #[test]
    fn test_path_len() {
        let d = Day12::load("examples/day12_example1.txt");
        assert_eq!(d.path_len(), 31);
    }

    #[test]
    fn test_rev_path_len() {
        let d = Day12::load("examples/day12_example1.txt");
        assert_eq!(d.rev_path_len(), 29);
    }
}
