use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct Day7 {
    session: Vec<String>,
}

impl Day7 {
    pub fn load(filename: &str) -> Day7 {
        let mut session: Vec<String> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            session.push(l.to_string());
        }

        Day7 { session }
    }

    fn run_session(&self) -> HashMap<String, usize> {
        // TODO: Create hash of path -> size.
        // Keep a stack of paths between root and cwd.
        // When a file is encountered, add its size to all paths on the stack.

        let mut dir_sizes: HashMap<String, usize> = HashMap::new();
        let mut path_stack: Vec<String> = Vec::new();

        lazy_static! {
            static ref FILE_RE: Regex =
                Regex::new(r"([0-9]+) ([a-zA-Z0-9\.]+)").unwrap();
            static ref CD_RE: Regex = 
                Regex::new(r"^\$ cd ([a-zA-Z0-1_]+)").unwrap();  // cap[1] is the directory being entered.
        }

        // Session context
        let mut cwd = String::new();
        cwd += "/";
        dir_sizes.insert(cwd.to_string(), 0);
        path_stack.push(cwd.to_string());

        for line in &self.session {
            if line.starts_with("$ ls") {
                // ls command (can ignore this)
                ()
            }
            else if line.starts_with("$ cd /") {
                // cd to top dir
                cwd.clear();
                cwd += "/";
                path_stack.clear();
                path_stack.push(cwd.to_string());
                // println!("cd to root");
            }
            else if line.starts_with("$ cd ..") {
                // cd to parent dir
                path_stack.pop();
                cwd.clear();
                cwd += path_stack.last().unwrap();
                // println!("cd ..")
            }
            else if line.starts_with("$ cd ") {
                let m = CD_RE.captures(line);
                match m {
                    Some(cap) => {
                        let subdir = cap[1].to_string();
                        cwd += &subdir;
                        cwd += "/";
                        // println!("cd into {subdir}, {cwd}");
                        path_stack.push(cwd.to_string());
                        if !dir_sizes.contains_key(&cwd) {
                            dir_sizes.insert(cwd.to_string(), 0);
                        }
                    }
                    None => {
                        // println!("{line}");
                        panic!("No subdir captured!");
                    }
                }
                // cd into a subdir
            }
            else if line.starts_with("dir ") {
                // a subdirectory has been observed. (ignore this)
            }
            else {
                // this is a file, record it
                // TODO-DW : Add size to all parent directories in pathStack
                let m = FILE_RE.captures(&line);
                match m {
                    Some(cap) => {
                        let size = cap[1].parse::<usize>().unwrap();
                        let _name = cap[2].to_string();
                        for path in &path_stack {
                            *dir_sizes.get_mut(path).unwrap() += size;
                            // println!("Adding {} to {}", size, path);
                        }
                    }
                    None => ()
                }
            }

        }

        dir_sizes
    }
}

impl Day for Day7 {
    fn part1(&self) -> Answer {
        let dir_sizes = self.run_session();
        let mut sum = 0;
        for (_dir, size) in &dir_sizes {
            if *size <= 100000 {
                sum += *size;
            }
        }

        Answer::Number(sum)
    }

    fn part2(&self) -> Answer {
        let total = 70000000;
        let needed = 30000000;
        let dir_sizes = self.run_session();
        let used = dir_sizes["/"];
        let free = total - used;
        let need_to_free = needed - free;

        // Find the directory with the smallest size >= need_to_free
        let mut can_free = used;
        for (_d, dir_size) in dir_sizes {
            if (dir_size >= need_to_free) & (dir_size <= can_free) {
                // We have a new best candidate to delete
                can_free = dir_size;
            }
        }

        Answer::Number(can_free)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day7::load("examples/day7_example1.txt");
        assert_eq!(d.session.len(), 23);
    }

    #[test]
    fn test_run_session() {
        let d = Day7::load("examples/day7_example1.txt");
        let dir_sizes = d.run_session();
        assert_eq!(dir_sizes.len(), 4);
    }

    #[test]
    fn test_part1() {
        let d = Day7::load("examples/day7_example1.txt");
        assert_eq!(d.part1(), Answer::Number(95437));
    }

    #[test]
    fn test_part2() {
        let d = Day7::load("examples/day7_example1.txt");
        assert_eq!(d.part2(), Answer::Number(24933642));
    }
}
