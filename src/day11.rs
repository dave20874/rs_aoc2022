use crate::day::{Day, Answer};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

#[derive (Copy, Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
    Double,
}

impl Operation {
    fn evaluate(&self, old: usize) -> usize {
        match self {
            Operation::Add(value) => {
                // println!("Worry level is increased by {} to {}", value, old+value);
                old + value
            }
            Operation::Mul(value) => {
                // println!("Worry level is multiplied by {} to {}", value, old*value);
                old * value
            }
            Operation::Square => {
                // println!("Worry level is multipled by itself to {}", old * old);
                old * old
            }
            Operation::Double => {
                // println!("Worry level is added to itself to {}", old + old);
                old + old
            }
        }
    }
}

struct Monkey {
    id: usize,
    items: VecDeque<usize>,
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
            items:VecDeque::new(),
            op:Operation::Add(0),
            divisor: 1,
            throw_true: 0,
            throw_false: 0,
            inspects: 0,
        }
    }

    fn set_items(&mut self, items: &Vec<usize>) {
        for i in items {
            self.items.push_back(*i);
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

    // throw() performs turn-based modifications on thrower:
    //   * removes head item from items
    //   * increments inspects
    //   * computes new worry value
    //   * determines id of monkey to throw to
    //   * returns catching monkey and new worry value of this item
    fn throw(&mut self, div3: bool) -> (usize, usize) {
        let mut worry = self.items.pop_front().unwrap();

        self.inspects += 1;

        // perform monkey's operation on item to get worry level.
        // println!("  monkey inspects an item with worry level {}", worry);
        worry = self.op.evaluate(worry);
        // println!("  worry level changes to {}", worry);

        if div3 {
            // reduce worry level, dividing by three.
            worry = worry / 3;
            // println!("  worry level is divided by 3 to {}", worry);
        }

        // decide which other monkey to throw item to
        let other_id = if worry % self.divisor == 0 {
            // println!("  current worry level is divisible by {}", self.divisor);
            self.throw_true
        }
        else {
            // println!("  current level is not divisible by {}", self.divisor);
            self.throw_false
        };
        // println!("  Item with worry level {} is thrown to {}", worry, other_id);

        (other_id, worry)
    }

    // catch() performs turn-based modifications on catcher:
    //   * adds new item to tail of items.
    fn catch(&mut self, worry: usize) {
        self.items.push_back(worry);
    }
}

struct Sim {
    monkeys: Vec<Monkey>,
    div3: bool,
    lcm: usize,
}

impl Sim {
    fn new(initial: &Vec<Monkey>, div3: bool) -> Sim {
        // Create a new vector of monkeys
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut lcm = 1;

        // Create new monkeys, copies of the starter ones
        for m in initial {
            let mut new_items = VecDeque::new();
            for item in &m.items {
                new_items.push_back(*item);
            }
            let new_monkey = Monkey {
                id: m.id,
                items: new_items,
                op: m.op,
                divisor: m.divisor,
                throw_true: m.throw_true,
                throw_false: m.throw_false,
                inspects: 0,
            };

            lcm = lcm * new_monkey.divisor;

            monkeys.push(new_monkey);
        }

        Sim { monkeys, div3, lcm }
    }

    fn do_monkey(&mut self, monkey_id: usize) {
        while !self.monkeys[monkey_id].items.is_empty() {
            let thrower = &mut self.monkeys[monkey_id];
            let (catcher_id, worry) = thrower.throw(self.div3);
            let adjusted_worry = worry % self.lcm;
            let catcher = &mut self.monkeys[catcher_id];
            catcher.catch(adjusted_worry);
        }
    }

    fn do_round(&mut self) {
        for id in 0..self.monkeys.len() {
            self.do_monkey(id);
        }
    }

    fn monkey_business(&self) -> usize {
        // collect inspect values into an array
        let mut inspects: Vec<usize> = Vec::new();
        for m in &self.monkeys {
            inspects.push(m.inspects);
        }

        // sort
        inspects.sort();
        inspects.reverse();

        // multiply the two largest terms
        // println!("inspects: {:?}", inspects);

        inspects[0] * inspects[1]
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
                Regex::new("new = old \\* ([0-9]+)").unwrap();
            static ref ADD_RE: Regex =
                Regex::new("new = old \\+ ([0-9]+)").unwrap();
            static ref SQUARE_RE: Regex =
                Regex::new("new = old \\* old").unwrap();
            static ref DOUBLE_RE: Regex =
                Regex::new("new = old \\+ old").unwrap();
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
        let mut sim = Sim::new(&self.monkeys, true);
        for _ in 0..20 {
            sim.do_round();
        }

        let mb = sim.monkey_business();

        Answer::Number(mb)
    }

    fn part2(&self) -> Answer {
        let mut sim = Sim::new(&self.monkeys, false);
        for _ in 0..10000 {
            sim.do_round();
        }

        let mb = sim.monkey_business();

        Answer::Number(mb)
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

    #[test]
    fn test_round() {
        let d = Day11::load("examples/day11_example1.txt");
        let mut sim: Sim = Sim::new(&d.monkeys, true);
        sim.do_round();
        assert_eq!(sim.monkeys[0].items.len(), 4);
        assert_eq!(sim.monkeys[1].items.len(), 6);
        assert_eq!(sim.monkeys[2].items.len(), 0);
        assert_eq!(sim.monkeys[3].items.len(), 0);
    }

    #[test]
    fn test_round20() {
        let d = Day11::load("examples/day11_example1.txt");
        let mut sim = Sim::new(&d.monkeys, true);
        for n in 0..20 {
            println!("--- Round {} -----------------------------------", n+1);
            sim.do_round();
        }

        let mb = sim.monkey_business();

        assert_eq!(mb, 10605);
    }


    #[test]
    fn test_round_pt2() {
        let d = Day11::load("examples/day11_example1.txt");
        let mut sim: Sim = Sim::new(&d.monkeys, false);
        sim.do_round();
        assert_eq!(sim.monkeys[0].items.len(), 4);
        assert_eq!(sim.monkeys[1].items.len(), 6);
        assert_eq!(sim.monkeys[2].items.len(), 0);
        assert_eq!(sim.monkeys[3].items.len(), 0);

        let mb = sim.monkey_business();
        assert_eq!(mb, 6*4);
    }

    #[test]
    fn test_rounds_pt2() {
        let d = Day11::load("examples/day11_example1.txt");
        let mut sim: Sim = Sim::new(&d.monkeys, false);
        /*
        for _ in 0..1000 {
            sim.do_round();
        }

        let mb = sim.monkey_business();
        assert_eq!(mb, 5204*5192);
        */

        for _ in 0..10000 {
            sim.do_round();
        }
        let mb = sim.monkey_business();
        assert_eq!(mb, 52166*52013);

    }
}
