 #[derive(PartialEq)]
 #[derive(Debug)]
 pub enum Answer {
    Number(usize),
    Message(String),
}

pub trait Day {
    fn part1(&self) -> Answer;
    fn part2(&self) -> Answer;
}
