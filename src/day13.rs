use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;
use core::cmp::{PartialOrd, PartialEq};

enum PacketComponent {
    List(PacketList),
    Integer(usize),
}

struct PacketList {
    list: Vec<PacketComponent>,
}

impl PartialEq for PacketComponent {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for PacketComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            PacketComponent::List(l1) => {
                match other {
                    PacketComponent::List(l2) => {
                        // Comparing two lists: l1, l2
                        l1.partial_cmp(l2)
                    }
                    PacketComponent::Integer(n2) => {
                        // Exactly one is integer, convert it to a list and compare the lists
                        let l2 = PacketList::from_int(*n2);
                        l1.partial_cmp(&l2)
                    }
                }
            }
            PacketComponent::Integer(n1) => {
                match other {
                    PacketComponent::List(l2) => {
                        // Exactly one is integer, convert it to a list and compare the lists
                        let l1 = PacketList::from_int(*n1);
                        l1.partial_cmp(&l2)
                    }
                    PacketComponent::Integer(n2) => {
                        // Comparing two integers
                        if n1 < n2 {
                            Some(Ordering::Less)
                        }
                        else if n1 > n2 {
                            Some(Ordering::Greater)
                        }
                        else {
                            Some(Ordering::Equal)
                        }
                    }
                }
            }
        }
    }
}

impl PacketList {
    // parse a &str into a PacketList
    pub fn new(s: &str) -> PacketList {
        let s_chars = s.chars().collect();
        let (_index, list) = PacketList::parse(&s_chars, 0);

        list        
    }

    pub fn from_int(n: usize) -> PacketList {
        let list = vec!(PacketComponent::Integer(n));
        PacketList { list }
    }

    fn parse(s_chars: &Vec<char>, mut index: usize) -> (usize, PacketList) {
        // create an empty list
        let mut list = Vec::new();

        // consume the opening '['
        assert_eq!('[', s_chars[index]);
        println!(" consume {}: '{}'", index, s_chars[index]);
        index += 1;

        loop {
            if s_chars[index] == '[' {
                // do the recursion thing
                println!("[ : recurse");
                let (new_index, sub_list) = PacketList::parse(s_chars, index);
                // assert_eq!(s_chars[new_index], ']');
                // println!(" consume {}: '{}'", new_index, s_chars[new_index]);
                index = new_index;

                // push sub_list into list
                list.push(PacketComponent::List(sub_list));
            }
            else if s_chars[index] == ']' {
                // pop out of recursion
                println!("consume {}: '{}'", index, s_chars[index]);
                println!("] : exit recursive call.");
                return (index+1, PacketList{list});
            }
            else if s_chars[index].is_ascii_digit() {
                // collect digits into a number
                let mut value = 0;
                while s_chars[index].is_ascii_digit() {
                    value *= 10;
                    value += s_chars[index].to_digit(10).unwrap() as usize;
                    println!(" consume {}: '{}'", index, s_chars[index]);
                    index += 1;
                }
                // println!(" unconsume {}: '{}'", index, s_chars[index]);
                // index -= 1;

                // store value in list
                list.push(PacketComponent::Integer(value));
            }
            else if s_chars[index] == ',' {
                // We can ignore these
                println!(" consume {}: '{}'", index, s_chars[index]);
                index += 1;
            }
            else {
                panic!("Unexpected character in packet list.");
            }
        }

    }
}

impl PartialEq for PacketList {
    fn eq(&self, other: &Self) -> bool {
        if self.list.len() != other.list.len() { return false; }
        for i in 0..self.list.len() {
            if self.list[i] != other.list[i] { return false; }
        }
        
        return true;
    }
}

impl PartialOrd for PacketList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut i = 0;
        while i < self.list.len() && i < other.list.len() {
            // compare ith items
            if self.list[i] < other.list[i] {
                return Some(Ordering::Less);
            }
            if self.list[i] > other.list[i] {
                return Some(Ordering::Greater);
            }

            i += 1;
        }

        // one or both lists ended.
        if (i >= self.list.len()) && (i >= other.list.len()) {
            // both lists ended
            Some(Ordering::Equal)
        }
        else if i >= self.list.len() {
            // self.list ended before other.list
            Some(Ordering::Less)
        }
        else {
            // other.list ended before self.list
            Some(Ordering::Greater)
        }

    }

}

pub struct Day13 {
    pairs: Vec<(PacketList, PacketList)>,
}

impl Day13 {
    pub fn load(filename: &str) -> Day13 {
        let mut pairs: Vec<(PacketList, PacketList)> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut packet1: Option<PacketList> = None;

        for line in reader.lines() {

            match line {
                Ok(line) => {
                    println!("parsing {}", line);
                    let trimmed = line.trim();
                    if trimmed.len() > 0 {
                        match packet1 {
                            Some(p1) => {
                                let p2 = PacketList::new(trimmed);
                                let pair = (p1, p2);
                                pairs.push(pair);
                                packet1 = None;
                            }
                            None => {
                                packet1 = Some(PacketList::new(trimmed));
                            }
                        }
                        
                    }
                }
                Err(_) => {
                    // Reached end of file.
                }
            }
        }
/*
        while reader.()) {
            let mut line = String::new();
            reader.read_line(line).unwrap();
            line.trim();
            if line.len() == 0 {
                break;
            }
            let packet1 = PacketList::new(line);

            line.clear();
            reader.read_line(line).unwrap();
            line.trim();
            if line.len() == 0 {
                break;
            }
            let packet2 = PacketList::new(line);

            let pair = (packet1, packet2);
            pairs.push(pair);
        }
        */

        Day13 { pairs }
    }

    pub fn ordered_right_sum(&self) -> usize {
        let mut sum = 0;
        let mut index = 0;

        for (left, right) in &self.pairs {
            index += 1;
            if left < right {
                sum += index;
            }
        }

        sum
    }
}

impl Day for Day13 {
    fn part1(&self) -> Answer {
        Answer::Number(self.ordered_right_sum())
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
        let d = Day13::load("examples/day13_example1.txt");
        assert_eq!(d.pairs.len(), 8);
    }

    #[test]
    fn test_ordering_eg1() {
        let p1 = PacketList::new("[1,1,3,1,1]");
        let p2 = PacketList::new("[1,1,5,1,1]");

        assert!(p1 < p2);
    }

    #[test]
    fn test_ordering_eg2() {
        let p1 = PacketList::new("[[1],[2,3,4]]");
        let p2 = PacketList::new("[[1],4]");

        assert!(p1 < p2);
    }

    #[test]
    fn test_ordering_eg3() {
        let p1 = PacketList::new("[9]");
        let p2 = PacketList::new("[[8,7,6]]");

        assert!(p1 >p2);
    }

    #[test]
    fn test_ordering_eg4() {
        let p1 = PacketList::new("[[4,4],4,4]");
        let p2 = PacketList::new("[[4,4],4,4,4]");

        assert!(p1 < p2);
    }

    #[test]
    fn test_ordering_eg5() {
        let p1 = PacketList::new("[7,7,7,7]");
        let p2 = PacketList::new("[7,7,7]");

        assert!(p1 > p2);
    }

    #[test]
    fn test_ordering_eg6() {
        let p1 = PacketList::new("[]");
        let p2 = PacketList::new("[3]");

        assert!(p1 < p2);
    }

    #[test]
    fn test_ordering_eg7() {
        let p1 = PacketList::new("[[[]]]");
        let p2 = PacketList::new("[[]]");

        assert!(p1 > p2);
    }

    #[test]
    fn test_ordering_eg8() {
        let p1 = PacketList::new("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let p2 = PacketList::new("[1,[2,[3,[4,[5,6,0]]]],8,9]");

        assert!(p1 > p2);
    }

    #[test]
    fn test_ordered_right_sum() {
        let d = Day13::load("examples/day13_example1.txt");
        let sum = d.ordered_right_sum();
        assert_eq!(sum, 13);
    }


    #[test]
    fn test_part1() {
        let d = Day13::load("examples/day13_example1.txt");
        let result = d.part1();
        assert_eq!(result, Answer::Number(13));
    }
}
