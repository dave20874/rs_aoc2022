use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};

enum PacketComponent {
    List(PacketList),
    Integer(usize),
}

/* 
impl PartialOrd for PacketComponent {

}
*/

struct PacketList {
    list: Vec<PacketComponent>,
}

impl PacketList {
    // parse a &str into a PacketList
    pub fn new(s: &str) -> PacketList {
        let s_chars = s.chars().collect();
        let (_index, list) = PacketList::parse(&s_chars, 0);

        list        
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

/* 
impl PartialOrd for PacketList {

}
*/

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
}

impl Day for Day13 {
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
        let d = Day13::load("examples/day13_example1.txt");
        assert_eq!(d.pairs.len(), 8);
    }
}
