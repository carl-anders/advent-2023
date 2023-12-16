use crate::helpers::BorrowTwo;

use super::day::Day;
use anyhow::Result;

fn predict_future(mut history: Vec<i64>) -> i64 {
    let mut sum = 0;
    while !history.iter().all(|&i| i == 0) {
        for i in 0..(history.len()-1) {
            let (left, right) = history.borrow_two(i, i+1);
            *left = *right - *left;
        }
        sum += history.pop().unwrap();
    }
    sum
}

pub struct Day9;
impl Day for Day9 {
    type Parsed = Vec<Vec<i64>>;
    type Output = i64;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
            .collect())
    }
    fn first(histories: Self::Parsed) -> Self::Output {
        histories
            .into_iter()
            .map(predict_future)
            .sum()
    }
    fn second(histories: Self::Parsed) -> Self::Output {
        histories
            .into_iter()
            .map(|mut history| {
                history.reverse();
                predict_future(history)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    fn parsed() -> <Day9 as Day>::Parsed {
        Day9::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day9::first(parsed()), 114);
    }
    #[test]
    fn part2() {
        assert_eq!(Day9::second(parsed()), 2);
    }
}
