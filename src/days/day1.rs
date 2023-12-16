use crate::helpers::FirstAndLast;

use super::day::Day;
use anyhow::Result;

pub struct Day1;
impl Day for Day1 {
    type Parsed = String;
    type Output = i32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input)
    }
    fn first(input: Self::Parsed) -> Self::Output {
        input
            .lines()
            .map(|line| {
                let mut it = line.chars().filter(char::is_ascii_digit);
                let first = it.next().unwrap();
                let last = if let Some(last) = it.next_back() {
                    last
                } else {
                    first
                };
                format!("{first}{last}").parse::<i32>().unwrap()
            })
            .sum()
    }
    fn second(input: Self::Parsed) -> Self::Output {
        let numbers = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        input
            .lines()
            .map(|line| {
                let mut fl = FirstAndLast::new();
                for pos in 0..usize::MAX {
                    let slice = &line[pos..];
                    if let Some(char) = slice.chars().next() {
                        match char {
                            '0'..='9' => {
                                fl.push(char.to_digit(10).unwrap() as usize);
                            }
                            _ => {
                                for (i, number) in numbers.iter().enumerate() {
                                    if slice.starts_with(number) {
                                        fl.push(i);
                                    }
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
                let (first, last) = fl.get().unwrap();
                format!("{first}{last}").parse::<i32>().unwrap()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    fn parsed() -> <Day1 as Day>::Parsed {
        Day1::parse(INPUT.to_string()).unwrap()
    }
    fn parsed2() -> <Day1 as Day>::Parsed {
        Day1::parse(INPUT2.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day1::first(parsed()), 142);
    }
    #[test]
    fn part2() {
        assert_eq!(Day1::second(parsed2()), 281);
    }
}
