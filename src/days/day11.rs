use std::collections::HashSet;

use crate::helpers::grid2d::Position2D;

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Universe {
    galaxies: Vec<Position2D<usize>>,
    empty_x: Vec<usize>,
    empty_y: Vec<usize>,
}
impl Universe {
    fn expand(&mut self, factor: usize) {
        for galaxy in &mut self.galaxies {
            let add_x = self.empty_x.iter().filter(|&&ex| ex < galaxy.x).count();
            let add_y = self.empty_y.iter().filter(|&&ey| ey < galaxy.y).count();

            *galaxy = galaxy.add_x(add_x * factor).add_y(add_y * factor);
        }
    }
    fn distances(&self) -> usize {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a.manhattan(b))
            .sum()
    }
}

pub struct Day11;
impl Day for Day11 {
    type Parsed = Universe;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let width = input.lines().next().unwrap().len();

        let mut empty_y = HashSet::new();
        let mut empty_x: HashSet<usize> = (0..width).collect();
        let mut galaxies = Vec::new();
        for (y, line) in input.lines().enumerate() {
            empty_y.insert(y);
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    empty_x.remove(&x);
                    empty_y.remove(&y);
                    galaxies.push(Position2D::new(x, y));
                }
            }
        }

        Ok(Universe {
            galaxies,
            empty_x: empty_x.drain().collect(),
            empty_y: empty_y.drain().collect(),
        })
    }
    fn first(mut universe: Self::Parsed) -> Self::Output {
        universe.expand(1);

        universe.distances()
    }
    fn second(mut universe: Self::Parsed) -> Self::Output {
        universe.expand(1_000_000 - 1);

        universe.distances()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    fn parsed() -> <Day11 as Day>::Parsed {
        Day11::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day11::first(parsed()), 374);
    }
    #[test]
    fn part2() {
        assert_eq!(Day11::second(parsed()), 82000210);
    }
}
