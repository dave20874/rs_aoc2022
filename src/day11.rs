use crate::day::{Day, Answer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

enum Operation {
    Add(usize),
    Mul(usize),
    Square,
    Double,
}

impl Operation {
    fn evaluate(&self, old: usize) -> usize {
        match self {
            Operation::Add(value) => old + value,
            Operation::Mul(value) => old * value,
            Operation::Square => old * old,
            Operation::Double => old + old,
        }
    }
}

struct Monkey {
    id: usize,
    items: Vec<usize>,
    op: Operation,
    divisor: usize,
    throw_true: usize,
    throw_false: usize,
    inspects: usize,
}

impl Monkey {
    fn new(id: usize) -> Monkey {
        Monkey {
            id,
            items:Vec::new(),
            op:Operation::Add(0),
            divisor: 1,
            throw_true: 0,
            throw_false: 0,
            inspects: 0,
        }
    }

    fn set_items(&mut self, items: &Vec<usize>) {
        for i in items {
            self.items.push(*i);
        }
    }

    fn set_op(&mut self, op: Operation) {
        self.op = op;
    }

    fn set_test(&mut self, divisor: usize) {
        self.divisor = divisor;
    }

    fn set_throw_true(&mut self, other: usize) {
        self.throw_true = other;
    }

    fn set_throw_false(&mut self, other: usize) {
        self.throw_false = other;
    }
}

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

impl Day11 {
    pub fn load(filename: &str) -> Day11 {
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut monkey: Option<Monkey> = None;
        lazy_static! {
            static ref MONKEY_RE: Regex =
                Regex::new("Monkey ([0-9]+):").unwrap();
            static ref STARTING_RE: Regex =
                Regex::new("  Starting items: (.*)").unwrap();
            static ref OPERATION_RE: Regex =
                Regex::new("  Operation: (.*)").unwrap();
            static ref TEST_RE: Regex =
                Regex::new("  Test: divisible by ([0-9]+)").unwrap();
            static ref THROW_TRUE_RE: Regex =
                Regex::new("    If true: throw to monkey ([0-9]+)").unwrap();
            static ref THROW_FALSE_RE: Regex =
                Regex::new("    If false: throw to monkey ([0-9]+)").unwrap();
            static ref MULTIPLY_RE: Regex =
                Regex::new("new = old * ([0-9]+)").unwrap();
            static ref ADD_RE: Regex =
                Regex::new("new = old + ([0-9]+)").unwrap();
            static ref SQUARE_RE: Regex =
                Regex::new("new = old * old").unwrap();
            static ref DOUBLE_RE: Regex =
                Regex::new("new = old + old").unwrap();
        }

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();

            // Match "Monkey N:"
            let caps = MONKEY_RE.captures(&l);
            match caps {
                Some(caps) => {
                    // New monkey starting
                    // store the last monkey we were working with
                    match monkey {
                        Some(m) => {
                            // println!("Pushing monkey.");
                            monkeys.push(m);
                        }
                        None => {}
                    }

                    // Create new monkey with id
                    let id = caps[1].parse::<usize>().unwrap();
                    monkey = Some(Monkey::new(id));
                }
                None => {}
            }

            match &mut monkey {
                None => {},
                Some(monkey) => {
                    // Match "Starting items"
                    let caps = STARTING_RE.captures(&l);
                    match caps {
                        Some(caps) => {
                            // caps[1] is a list of worry values, "65, 79, 98, ..."
                            let mut start_list: Vec<usize> = Vec::new();
                            for item in caps[1].split(", ") {
                                // println!("Parsing as int: '{}'", item);
                                start_list.push(item.parse::<usize>().unwrap());
                            }
                            monkey.set_items(&start_list);
                        }
                        None => {}
                    }

                    // Match "Operation"
                    let caps2 = OPERATION_RE.captures(&l);
                    match caps2 {
                        Some(caps2) => {
                            let op_str: &str = &caps2[1];

                            // Set operation
                            match MULTIPLY_RE.captures(&op_str) {
                                Some(op_caps) => {
                                    let constant = op_caps[1].parse::<usize>().unwrap();
                                    monkey.set_op(Operation::Mul(constant));
                                }
                                None => {}
                            }
                            match ADD_RE.captures(&op_str) {
                                Some(op_caps) => {
                                    let constant = op_caps[1].parse::<usize>().unwrap();
                                    monkey.set_op(Operation::Add(constant));
                                }
                                None => {}
                            }
                            match SQUARE_RE.captures(&op_str) {
                                Some(_op_caps) => {
                                    monkey.set_op(Operation::Square);
                                }
                                None => {}
                            }
                            match DOUBLE_RE.captures(&op_str) {
                                Some(_op_caps) => {
                                    monkey.set_op(Operation::Double);
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }

                    // Match "Test"
                    let caps = TEST_RE.captures(&l);
                    match caps {
                        Some(caps) => {
                            let divisor = caps[1].parse::<usize>().unwrap();
                            monkey.set_test(divisor);
                        }
                        None => {}
                    }

                    // Match "If true"
                    let caps = THROW_TRUE_RE.captures(&l);
                    match caps {
                        Some(caps) => {
                            let other = caps[1].parse::<usize>().unwrap();
                            monkey.set_throw_true(other);
                        }
                        None => {}
                    }

                    // Match "If false"
                    let caps = THROW_FALSE_RE.captures(&l);
                    match caps {
                        Some(caps) => {
                            let other = caps[1].parse::<usize>().unwrap();
                            monkey.set_throw_false(other);
                        }
                        None => {}
                    }
                }
            }
        }

        // Store the last monkey under construction
        match monkey {
            Some(m) => {
                // println!("Pushing last monkey.");
                monkeys.push(m);
            }
            None => {}
        }

        Day11 { monkeys }
    }
}

impl Day for Day11 {
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
        let d = Day11::load("examples/day11_example1.txt");
        assert_eq!(d.monkeys.len(), 4);
    }
}
