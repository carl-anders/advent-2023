use std::collections::HashMap;

use super::day::Day;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Card {
    id: usize,
    winning: Vec<u8>,
    numbers: Vec<u8>,
}

pub struct Day4;
impl Day for Day4 {
    type Parsed = Vec<Card>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let (id, numbers) = line.split_once(':').unwrap();
                let id = id.rsplit_once(' ').unwrap().1.parse().unwrap();

                let (winning, numbers) = numbers.split_once('|').unwrap();
                Card {
                    id,
                    winning: winning.split(' ').filter_map(|n| n.parse().ok()).collect(),
                    numbers: numbers.split(' ').filter_map(|n| n.parse().ok()).collect(),
                }
            })
            .collect())
    }
    #[allow(clippy::cast_possible_truncation)]
    fn first(cards: Self::Parsed) -> Self::Output {
        cards
            .iter()
            .map(|card| {
                let found = card
                    .numbers
                    .iter()
                    .filter(|i| card.winning.contains(i))
                    .count() as u32;
                if found > 0 {
                    2usize.pow(found - 1)
                } else {
                    0
                }
            })
            .sum()
    }
    fn second(cards: Self::Parsed) -> Self::Output {
        let mut sum = 0;

        let mut copies: HashMap<usize, usize> = HashMap::new();
        for card in &cards {
            let self_copies = *copies.get(&card.id).unwrap_or(&0);
            sum += 1 + self_copies;

            let found = card
                .numbers
                .iter()
                .filter(|i| card.winning.contains(i))
                .count();
            if found > 0 {
                for i in 1..=found {
                    let id = card.id + i;
                    *copies.entry(id).or_default() += 1 + self_copies;
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    fn parsed() -> <Day4 as Day>::Parsed {
        Day4::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day4::first(parsed()), 13);
    }
    #[test]
    fn part2() {
        assert_eq!(Day4::second(parsed()), 30);
    }
}
