use crate::day::{Day, Answer};
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet};
use priority_queue::PriorityQueue;

pub struct Day18 {
    cubes: HashSet<(isize, isize, isize)>,
}

impl Day18 {
    pub fn load(filename: &str) -> Day18 {
        let mut cubes: HashSet<(isize, isize, isize)> = HashSet::new();
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("([0-9]+),([0-9]+),([0-9]+)").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            let caps = LINE_RE.captures(&l);
            match caps {
                Some(caps) => {
                    let x: isize = caps[1].parse::<isize>().unwrap();
                    let y: isize = caps[2].parse::<isize>().unwrap();
                    let z: isize = caps[3].parse::<isize>().unwrap();
                    cubes.insert( (x, y, z) );
                }
                None => {}
            }
        }

        Day18 { cubes }
    }

    pub fn surface_area(cubes: &HashSet<(isize, isize, isize)>) -> usize {
        // start with area being the full area of all the cubes.
        let mut area = 6 * cubes.len();

        // Now, for every cube ...
        for (x, y, z) in cubes {
            // For every potential neighbor
            for neighbor in 
                [(*x+1, *y, *z), (*x-1, *y, *z), 
                 (*x, *y+1, *z), (*x, *y-1, *z),
                 (*x, *y, *z+1), (*x, *y, *z-1)] {
                    if cubes.contains(&neighbor) {
                        area -= 1;
                    }
            }
        }

        area
    }

    fn find_a_top(cubes: &HashSet<(isize, isize, isize)>) -> (isize, isize, isize) {
        let mut a_top: Option<(isize, isize, isize)> = None;

        for c in cubes {
            match a_top {
                Some(coord) => {
                    // This cube has the same x, y as the candidate but a greater z
                    // Replace our earlier candidate with this one.
                    if coord.0 == c.0 && coord.1 == c.1 && c.2 > coord.2 {
                        a_top = Some(*c);
                    }
                }
                None => {
                    // This is the first cube we've seen adopt it as the candidate top piece
                    a_top = Some(*c);
                }
            }
        }

        // Return a coordinate with z one higher than the found top.
        // That cube must be on the exterior shell, adjacent to the droplet.
        match a_top {
            Some(coord) => {
                (coord.0, coord.1, coord.2+1)
            }
            None => {
                assert!(false);
                (0, 0, 0)
            }
        }
    }

    fn is_adjacent(cube: (isize, isize, isize), cubes: &HashSet<(isize, isize, isize)>) -> bool {
        for n in Day18::adjacent_neighbors(cube) {
            if cubes.contains(&n) { 
                return true;
            }
        }

        false
    }

    fn adjacent_neighbors(cube: (isize, isize, isize)) -> [(isize, isize, isize); 6] {
        let (x, y, z) = cube;
        let neighbors: [(isize, isize, isize); 6] = [
            (x-1, y, z),
            (x+1, y, z),
            (x, y-1, z),
            (x, y+1, z),
            (x, y, z-1),
            (x, y, z+1),
        ];
        
        neighbors
    }

    pub fn exterior_area(cubes: &HashSet<(isize, isize, isize)>) -> usize {
        let mut shell: HashSet<(isize, isize, isize)> = HashSet::new();
        let mut to_explore: PriorityQueue<(isize, isize, isize, usize), Reverse<usize>> = PriorityQueue::new();
        let mut identified: HashSet<(isize, isize, isize)> = HashSet::new();
        
        // Find a cube in the shell (The shell is all cubes outside the droplet but adjacent to it) 
        // Add that cube and face to the search 
        let top = Day18::find_a_top(&cubes);
        println!("Starting from top: {:?}", top);
        to_explore.push((top.0, top.1, top.2, 0), Reverse(0));  // 0 : distance from droplet.

        while to_explore.len() > 0 {
            let ((x, y, z, mut dist), _priority) = to_explore.pop().unwrap();

            // If this is adjacent to the droplet, it's part of the shell
            // let mut next_distance = dist+1;
            if Day18::is_adjacent((x, y, z), cubes) {
                // println!("Adding {:?} to shell", (x, y, z));
                shell.insert((x, y, z));
                dist = 0;
                // next_distance = 2;
            }
            if dist <= 3 {
                for n in Day18::adjacent_neighbors((x, y, z)) {
                    if !cubes.contains(&n) { 
                        if !identified.contains(&n) {               
                            to_explore.push((n.0, n.1, n.2, dist+1), Reverse(dist+1));
                            identified.insert(n);
                        }
                    }
                }
            }
        }

        println!("Shell consists of {} cubes.", shell.len());

        let mut area = 0;

        // count the adjacencies between shell and droplet
        for shell_cube in shell {
            // println!("Contacts with {:?}", shell_cube);
            for n in Day18::adjacent_neighbors(shell_cube) {
                if cubes.contains(&n) {
                    area += 1;
                }
            }
        }

        area
    }
}

impl Day for Day18 {
    fn part1(&self) -> Answer {
        Answer::Number(Day18::surface_area(&self.cubes))
    }

    fn part2(&self) -> Answer {
        Answer::Number(Day18::exterior_area(&self.cubes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day18::load("examples/day18_example1.txt");
        assert_eq!(d.cubes.len(), 13);
    }

    #[test]
    fn test_area() {
        let d = Day18::load("examples/day18_example1.txt");
        assert_eq!(Day18::surface_area(&d.cubes), 64);
    }

    #[test]
    fn test_part1() {
        let d = Day18::load("examples/day18_example1.txt");
        assert_eq!(d.part1(), Answer::Number(64));
    }

    #[test]
    fn test_exterior_area() {
        let d = Day18::load("examples/day18_example1.txt");
        assert_eq!(Day18::exterior_area(&d.cubes), 58);
    }

    #[test]
    fn test_exterior_area_input() {
        let d = Day18::load("data_aoc2022/day18_input.txt");
        assert_eq!(Day18::exterior_area(&d.cubes), 2564);
    }

    #[test]
    fn test_part2() {
        let d = Day18::load("examples/day18_example1.txt");
        assert_eq!(d.part2(), Answer::Number(58));
    }

    #[test]
    fn test_part2_input() {
        let d = Day18::load("data_aoc2022/day18_input.txt");
        assert_eq!(d.part2(), Answer::Number(2564));
    }

}
