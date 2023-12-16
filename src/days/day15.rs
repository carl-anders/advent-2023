use super::day::Day;
use anyhow::Result;

pub struct Day15;
impl Day for Day15 {
    type Parsed = String;
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input)
    }
    fn first(_elves: Self::Parsed) -> Self::Output {
        5
    }
    fn second(_elves: Self::Parsed) -> Self::Output {
        5
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "";
    fn parsed() -> <Day15 as Day>::Parsed {
        Day15::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day15::first(parsed()), 24000);
    }
    #[test]
    fn part2() {
        assert_eq!(Day15::second(parsed()), 45000);
    }
}
*/