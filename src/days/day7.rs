#![allow(clippy::cast_lossless)]

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;

fn card_to_u32(input: char) -> u32 {
    match input {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => input.to_digit(10).unwrap(),
    }
}
#[derive(Debug, Clone)]
pub struct Hand([u32; 5]);
impl Hand {
    fn hand_type(&self) -> u64 {
        let counts = self.0.into_iter().counts();
        match counts.len() {
            1 => 7,
            2 => match *counts.values().next().unwrap() {
                1 | 4 => 6,
                _ => 5,
            },
            3 => match counts.values().max().unwrap() {
                3 => 4,
                _ => 3,
            },
            4 => 2,
            _ => 1,
        }
    }
    const fn card_score(&self) -> u64 {
        ((self.0[0] as u64) << 16)
            + ((self.0[1] as u64) << 12)
            + ((self.0[2] as u64) << 8)
            + ((self.0[3] as u64) << 4)
            + (self.0[4] as u64)
    }
    fn score(&self) -> u64 {
        (self.hand_type() << 20) + self.card_score()
    }
    fn replace_j_with(&self, replace: u32) -> Self {
        Self(self.0.map(|card| if card == 11 { replace } else { card }))
    }
    fn j_score(&self) -> u64 {
        let counts = self.0.into_iter().counts();
        match counts.get(&11).unwrap_or(&0) {
            0 => self.score(),
            5 => self.replace_j_with(1).score(),
            _ => {
                counts.iter().filter(|(num, _)| **num != 11).map(|(to_be_copied, _)| {
                    let hand = self.replace_j_with(*to_be_copied);
                    let score_hand = self.replace_j_with(1);
                    (hand.hand_type() << 20) + score_hand.card_score()
                }).max().unwrap()
            }
        }
    }
}

pub struct Day7;
impl Day for Day7 {
    type Parsed = Vec<(Hand, u32)>;
    type Output = u64;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                (
                    Hand(
                        hand.chars()
                            .map(card_to_u32)
                            .collect::<Vec<u32>>()
                            .try_into()
                            .unwrap(),
                    ),
                    bid.parse().unwrap(),
                )
            })
            .collect())
    }
    fn first(hands: Self::Parsed) -> Self::Output {
        let mut hands: Vec<(u64, u32)> = hands
            .into_iter()
            .map(|(hand, bid)| (hand.score(), bid))
            .collect();
        hands.sort_unstable_by_key(|(hand, _)| *hand);

        hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) as u64 * *bid as u64)
            .sum()
    }
    fn second(hands: Self::Parsed) -> Self::Output {
        let mut hands: Vec<(u64, u32)> = hands
            .into_iter()
            .map(|(hand, bid)| (hand.j_score(), bid))
            .collect();
        hands.sort_unstable_by_key(|(hand, _)| *hand);

        hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) as u64 * *bid as u64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    const INPUT2: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
    fn parsed() -> <Day7 as Day>::Parsed {
        Day7::parse(INPUT.to_string()).unwrap()
    }
    fn parsed2() -> <Day7 as Day>::Parsed {
        Day7::parse(INPUT2.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day7::first(parsed()), 6440);
        assert_eq!(Day7::first(parsed2()), 6592);
    }
    #[test]
    fn part2() {
        assert_eq!(Day7::second(parsed()), 5905);
        assert_eq!(Day7::second(parsed2()), 6839);
    }
}
