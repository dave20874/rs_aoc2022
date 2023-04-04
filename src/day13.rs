use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;
use core::cmp::{PartialOrd, PartialEq};
use sorted_vec::SortedVec;

enum PacketComponent {
    List(PacketList),
    Integer(usize),
}

struct PacketList {
    list: Vec<PacketComponent>,
}

impl Ord for PacketComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PacketComponent::List(l1) => {
                match other {
                    PacketComponent::List(l2) => {
                        // Comparing two lists: l1, l2
                        l1.cmp(l2)
                    }
                    PacketComponent::Integer(n2) => {
                        // Exactly one is integer, convert it to a list and compare the lists
                        let l2 = PacketList::from_int(*n2);
                        l1.cmp(&l2)
                    }
                }
            }
            PacketComponent::Integer(n1) => {
                match other {
                    PacketComponent::List(l2) => {
                        // Exactly one is integer, convert it to a list and compare the lists
                        let l1 = PacketList::from_int(*n1);
                        l1.cmp(&l2)
                    }
                    PacketComponent::Integer(n2) => {
                        // Comparing two integers
                        if n1 < n2 {
                            Ordering::Less
                        }
                        else if n1 > n2 {
                            Ordering::Greater
                        }
                        else {
                            Ordering::Equal
                        }
                    }
                }
            }
        }
    }
}

impl PartialEq for PacketComponent {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// This says PartialEq provides a total ordering.
impl Eq for PacketComponent {}

impl PartialOrd for PacketComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        // println!(" consume {}: '{}'", index, s_chars[index]);
        index += 1;

        loop {
            if s_chars[index] == '[' {
                // do the recursion thing
                // println!("[ : recurse");
                let (new_index, sub_list) = PacketList::parse(s_chars, index);
                // assert_eq!(s_chars[new_index], ']');
                // println!(" consume {}: '{}'", new_index, s_chars[new_index]);
                index = new_index;

                // push sub_list into list
                list.push(PacketComponent::List(sub_list));
            }
            else if s_chars[index] == ']' {
                // pop out of recursion
                // println!("consume {}: '{}'", index, s_chars[index]);
                // println!("] : exit recursive call.");
                return (index+1, PacketList{list});
            }
            else if s_chars[index].is_ascii_digit() {
                // collect digits into a number
                let mut value = 0;
                while s_chars[index].is_ascii_digit() {
                    value *= 10;
                    value += s_chars[index].to_digit(10).unwrap() as usize;
                    // println!(" consume {}: '{}'", index, s_chars[index]);
                    index += 1;
                }
                // println!(" unconsume {}: '{}'", index, s_chars[index]);
                // index -= 1;

                // store value in list
                list.push(PacketComponent::Integer(value));
            }
            else if s_chars[index] == ',' {
                // We can ignore these
                // println!(" consume {}: '{}'", index, s_chars[index]);
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

impl Eq for PacketList {}

impl Ord for PacketList {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut i = 0;
        while i < self.list.len() && i < other.list.len() {
            // compare ith items
            if self.list[i] < other.list[i] {
                return Ordering::Less;
            }
            if self.list[i] > other.list[i] {
                return Ordering::Greater;
            }

            i += 1;
        }

        // one or both lists ended.
        if (i >= self.list.len()) && (i >= other.list.len()) {
            // both lists ended
            Ordering::Equal
        }
        else if i >= self.list.len() {
            // self.list ended before other.list
            Ordering::Less
        }
        else {
            // other.list ended before self.list
            Ordering::Greater
        }

    }
}

impl PartialOrd for PacketList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
                    // println!("parsing {}", line);
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

    pub fn decode_key(&self) -> usize {
        let mut packets: SortedVec<&PacketList> = SortedVec::new();
        let divider1 = PacketList::new("[[2]]");
        let divider2 = PacketList::new("[[6]]");

        // throw divider packets into the empty packet list
        packets.insert(&divider1);  
        packets.insert(&divider2);

        for (left, right) in &self.pairs {
            packets.insert(left);
            packets.insert(right);
        }

        // Now get index of divider packets
        let div1_ref = &divider1;
        let div2_ref = &divider2;
        let index1 = packets.binary_search(&div1_ref).unwrap();
        let index2 = packets.binary_search(&div2_ref).unwrap();

        // decode key is (1-based) index1 * index2
        (index1+1) * (index2 + 1)
    }
}

impl Day for Day13 {
    fn part1(&self) -> Answer {
        Answer::Number(self.ordered_right_sum())
    }

    fn part2(&self) -> Answer {
        Answer::Number(self.decode_key())
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

    #[test]
    fn test_decoder_key() {
        let d = Day13::load("examples/day13_example1.txt");
        let result = d.decode_key();
        assert_eq!(result, 140);
    }

    #[test]
    fn test_part2() {
        let d = Day13::load("examples/day13_example1.txt");
        let result = d.part2();
        assert_eq!(result, Answer::Number(140));
    }
}
