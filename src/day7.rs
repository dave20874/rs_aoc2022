use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day7 {
    session: Vec<String>,
}

struct FileInfo {
    path: String,
    name: String,
    size: usize,
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

    fn run_session(&self) -> Vec<FileInfo> {
        let mut filesystem: Vec<FileInfo> = Vec::new();

        lazy_static! {
            static ref FILE_RE: Regex =
                Regex::new("([0-9]+) ([a-zA-Z0-9\\.]+)").unwrap();
        }

        // Session context
        let mut cwd = "/";
        for line in &self.session {
            if &line[0..1] == "$" {
                // Interpret a command
                
                // ...
            }
            else {
                // process file or dir name
                if line.starts_with("dir ") {
                    let name = &line[4..];
                    let path: String = cwd.to_string() + name;

                    // record this dir in filesystem under it's path with a filename of '.' and size of 0
                    let file_info = FileInfo { path: path, name: ".".to_string(), size: 0 };
                    filesystem.push(file_info);
                }
                else {
                    // this is a file, record it
                    let m = FILE_RE.captures(&line);
                    match m {
                        Some(cap) => {
                            let size = cap[1].parse::<usize>().unwrap();
                            let name = cap[2].to_string();
                            let file_info = FileInfo { path:cwd.to_string(), name:name, size:size };
                            filesystem.push(file_info);
                        }
                        None => ()
                    }
                }
                // ...
            }

        }

        filesystem
    }
}

impl Day for Day7 {
    fn part1(&self) -> Answer {
        Answer::Number(1)
    }

    fn part2(&self) -> Answer {
        Answer::Number(2)
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
        let fs = d.run_session();
        assert_eq!(fs.len(), 13);
    }
}
