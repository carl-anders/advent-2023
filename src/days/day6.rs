use super::day::Day;
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Race {
    time: i64,
    distance: i64,
}

pub struct Day6;
impl Day for Day6 {
    type Parsed = (Vec<Race>, Race);
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let (times, distances) = input.lines().collect_tuple().unwrap();
        let times = times.split_once(':').unwrap().1;
        let distances = distances.split_once(':').unwrap().1;

        let first = times
            .split_whitespace()
            .filter_map(|n| n.parse::<i64>().ok())
            .zip(
                distances
                    .split_whitespace()
                    .filter_map(|n| n.parse::<i64>().ok()),
            )
            .map(|(time, distance)| Race { time, distance })
            .collect();

        let second = Race {
            time: times.replace(' ', "").parse::<i64>().unwrap(),
            distance: distances.replace(' ', "").parse::<i64>().unwrap(),
        };

        Ok((first, second))
    }
    fn first((races, _): Self::Parsed) -> Self::Output {
        races
            .iter()
            .map(|race| {
                (1..race.time)
                    .map(|speed| {
                        let time_left = race.time - speed;
                        time_left * speed
                    })
                    .filter(|distance| distance > &race.distance)
                    .count()
            })
            .product()
    }
    fn second((_, race): Self::Parsed) -> Self::Output {
        (1..race.time)
            .map(|speed| {
                let time_left = race.time - speed;
                time_left * speed
            })
            .filter(|distance| distance > &race.distance)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    fn parsed() -> <Day6 as Day>::Parsed {
        Day6::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day6::first(parsed()), 288);
    }
    #[test]
    fn part2() {
        assert_eq!(Day6::second(parsed()), 71503);
    }
}
