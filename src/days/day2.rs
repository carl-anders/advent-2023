use enum_map::enum_map;
use enum_map::Enum;

use super::day::Day;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Game {
    plays: Vec<Vec<(u32, Color)>>,
    id: u32,
}
#[derive(Clone, Copy, Debug, Enum)]
pub enum Color {
    Red,
    Green,
    Blue,
}
pub struct Day2;
impl Day for Day2 {
    type Parsed = Vec<Game>;
    type Output = u32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let (id, games) = line.split_once(": ").unwrap();
                let id = id.split(' ').find_map(|s| s.parse::<u32>().ok()).unwrap();
                let games = games
                    .split("; ")
                    .map(|set| {
                        set.split(", ")
                            .map(|one| {
                                let (num, color) = one.split_once(' ').unwrap();
                                (
                                    num.parse::<u32>().unwrap(),
                                    match color {
                                        "red" => Color::Red,
                                        "green" => Color::Green,
                                        "blue" => Color::Blue,
                                        _ => panic!(),
                                    },
                                )
                            })
                            .collect()
                    })
                    .collect();
                Game { plays: games, id }
            })
            .collect())
    }
    fn first(games: Self::Parsed) -> Self::Output {
        games
            .iter()
            .filter(|game| {
                !game.plays.iter().any(|play| {
                    play.iter().any(|(num, color)| match color {
                        Color::Red => *num > 12,
                        Color::Green => *num > 13,
                        Color::Blue => *num > 14,
                    })
                })
            })
            .map(|game| game.id)
            .sum()
    }
    fn second(games: Self::Parsed) -> Self::Output {
        games
            .iter()
            .map(|game| {
                let mut min = enum_map! {
                    Color::Red => 0,
                    Color::Green => 0,
                    Color::Blue => 0
                };
                for play in &game.plays {
                    for &(num, color) in play {
                        if min[color] < num {
                            min[color] = num;
                        }
                    }
                }
                min.values().product::<u32>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    fn parsed() -> <Day2 as Day>::Parsed {
        Day2::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day2::first(parsed()), 8);
    }
    #[test]
    fn part2() {
        assert_eq!(Day2::second(parsed()), 2286);
    }
}
