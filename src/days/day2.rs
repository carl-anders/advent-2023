use super::day::Day;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Game {
    plays: Vec<Play>,
    id: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct Play {
    red: usize,
    green: usize,
    blue: usize,
}
pub struct Day2;
impl Day for Day2 {
    type Parsed = Vec<Game>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let (id, games) = line.split_once(": ").unwrap();
                let id = id.split(' ').find_map(|s| s.parse::<usize>().ok()).unwrap();
                let games = games
                    .split("; ")
                    .map(|set| {
                        let mut play = Play {
                            red: 0,
                            green: 0,
                            blue: 0,
                        };
                        for one in set.split(", ") {
                            let (num, color) = one.split_once(' ').unwrap();
                            let num = num.parse::<usize>().unwrap();
                            match color {
                                "red" => play.red = num,
                                "green" => play.green = num,
                                "blue" => play.blue = num,
                                _ => panic!(),
                            }
                        }
                        play
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
                !game
                    .plays
                    .iter()
                    .any(|play| play.red > 12 || play.green > 13 || play.blue > 14)
            })
            .map(|game| game.id)
            .sum()
    }
    fn second(games: Self::Parsed) -> Self::Output {
        games
            .iter()
            .map(|game| {
                [
                    game.plays.iter().map(|play| play.red).max().unwrap(),
                    game.plays.iter().map(|play| play.green).max().unwrap(),
                    game.plays.iter().map(|play| play.blue).max().unwrap(),
                ]
                .iter()
                .product::<usize>()
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
