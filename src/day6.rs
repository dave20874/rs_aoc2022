use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

pub struct Day6 {
    seq: Vec<char>,
}

impl Day6 {
    pub fn load(filename: &str) -> Day6 {
        let mut seq: Vec<char> = Vec::new();

        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let mut s: String = String::new();
        reader.read_line(&mut s).unwrap();

        let trimmed = s.trim_end();

        for c in trimmed.chars() {
            seq.push(c);
        }

        Day6 { seq }
    }

    fn find_no_repeat(&self, len:usize) -> usize {
        let mut marker: VecDeque<char> = VecDeque::new();

        for i in 0..len {
            marker.push_back(self.seq[i]);
        }

        let mut index = len;
        let mut found = false;

        while !found {


            // check to see if all marker values are unique
            found = true;
            for i in 0..marker.len()-1 {
                for j in i+1..marker.len() {
                    if marker[i] == marker[j] {
                        found = false;
                    }
                }
            }

            if !found {
                // remove oldest from marker
                marker.pop_front();
                // put next char in the marker buffer
                marker.push_back(self.seq[index]);
                index += 1;
            }
        }

        index
    }
}

impl Day for Day6 {
    fn part1(&self) -> Answer {
        Answer::Number(self.find_no_repeat(4))
    }

    fn part2(&self) -> Answer {
        Answer::Number(self.find_no_repeat(14))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::day::Day;

    #[test]
    fn test_load() {
        let d = Day6::load("examples/day6_example1.txt");
        assert_eq!(d.seq.len(), 30);
    }

    #[test]
    fn test_find_no_repeats() {
        let cases: Vec<(&str, usize, usize)> = vec!(
            ("examples/day6_example1.txt", 4, 7),
            ("examples/day6_example2.txt", 4, 5),  
            ("examples/day6_example3.txt", 4, 6),  
            ("examples/day6_example4.txt", 4, 10),  
            ("examples/day6_example5.txt", 4, 11),
            ("examples/day6_example1.txt", 14, 19),
            ("examples/day6_example2.txt", 14, 23),  
            ("examples/day6_example3.txt", 14, 23),  
            ("examples/day6_example4.txt", 14, 29),  
            ("examples/day6_example5.txt", 14, 26),    
        );

        for (filename, len, expected) in cases {
            let d = Day6::load(filename);
            let marker = d.find_no_repeat(len);
            assert_eq!(marker, expected)
        }
    }
}
