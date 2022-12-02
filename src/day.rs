pub trait Day {
    fn part1(&self) -> Result<usize, &str>;
    fn part2(&self) -> Result<usize, &str>;
}
