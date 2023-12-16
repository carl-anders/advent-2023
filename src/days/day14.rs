use std::{
    collections::{hash_map::Entry, HashMap},
    fmt,
    ops::Range,
};

use crate::helpers::grid2d::{parse_str_grid, Direction4Way};

use super::day::Day;
use anyhow::Result;
use itertools::Either;
use ndarray::{Array2, Axis};

type Dir = Direction4Way;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Place {
    RoundRock,
    CubeRock,
    Empty,
}
impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RoundRock => write!(f, "O"),
            Self::CubeRock => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}
#[allow(dead_code)]
fn print_dish(dish: &Array2<Place>) {
    for slice in dish.axis_iter(Axis(0)) {
        for p in slice {
            print!("{p}");
        }
        println!();
    }
}

fn reversable_range(
    range: Range<usize>,
    reverse: bool,
) -> Either<impl Iterator<Item = usize>, impl Iterator<Item = usize>> {
    if reverse {
        Either::Right(range.rev())
    } else {
        Either::Left(range)
    }
}

fn tilt(dish: &mut Array2<Place>, dir: Dir) {
    let axis = Axis(match dir {
        Dir::North | Dir::South => 1,
        Dir::East | Dir::West => 0,
    });
    let reverse = matches!(dir, Dir::South | Dir::East);
    for mut slice in dish.axis_iter_mut(axis) {
        let len = slice.len();
        let mut last_filled = if reverse { len } else { usize::MAX };
        for i in reversable_range(0..len, reverse) {
            match slice[i] {
                Place::RoundRock => {
                    last_filled = if reverse {
                        last_filled - 1
                    } else {
                        last_filled.wrapping_add(1)
                    };
                    if last_filled != i {
                        slice[last_filled] = Place::RoundRock;
                        slice[i] = Place::Empty;
                    }
                }
                Place::CubeRock => {
                    last_filled = i;
                }
                Place::Empty => {}
            }
        }
    }
}

fn cycle(dish: &mut Array2<Place>) {
    tilt(dish, Dir::North);
    tilt(dish, Dir::West);
    tilt(dish, Dir::South);
    tilt(dish, Dir::East);
}

fn calc_dish_load(dish: &Array2<Place>) -> usize {
    let height = dish.shape()[0];
    let sum = dish
        .indexed_iter()
        .map(|((y, _), place)| match place {
            Place::RoundRock => height - y,
            _ => 0,
        })
        .sum();
    sum
}

pub struct Day14;
impl Day for Day14 {
    type Parsed = Array2<Place>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(parse_str_grid(&input, |c| match c {
            'O' => Place::RoundRock,
            '#' => Place::CubeRock,
            _ => Place::Empty,
        })?)
    }
    fn first(mut dish: Self::Parsed) -> Self::Output {
        tilt(&mut dish, Direction4Way::North);
        calc_dish_load(&dish)
    }
    fn second(mut dish: Self::Parsed) -> Self::Output {
        let total_steps = 1_000_000_000;
        let mut states = HashMap::new();
        let mut step = 0;
        while step < total_steps {
            cycle(&mut dish);
            step += 1;
            match states.entry(dish.clone().into_raw_vec()) {
                Entry::Occupied(entry) => {
                    let loop_length = step - entry.get();
                    step += ((total_steps - step) / loop_length) * loop_length;
                    break;
                }
                Entry::Vacant(entry) => {
                    entry.insert(step);
                }
            }
        }
        for _ in step..total_steps {
            cycle(&mut dish);
        }
        calc_dish_load(&dish)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    fn parsed() -> <Day14 as Day>::Parsed {
        Day14::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day14::first(parsed()), 136);
    }
    #[test]
    fn part2() {
        assert_eq!(Day14::second(parsed()), 64);
    }
}
