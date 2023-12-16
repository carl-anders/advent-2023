use crate::helpers::grid2d::parse_str_grid;

use super::day::Day;
use anyhow::Result;
use ndarray::{Array2, Axis};

fn reflect(grid: &Array2<bool>, axis: Axis) -> Option<usize> {
    let size = grid.len_of(axis);
    for pos in 1..size {
        let mut reflects = true;
        let mut left = pos - 1;
        let mut right = pos;
        while reflects && left < size && right < size {
            if grid.index_axis(axis, left) != grid.index_axis(axis, right) {
                reflects = false;
            }

            left = left.wrapping_sub(1);
            right += 1;
        }
        if reflects {
            return Some(pos);
        }
    }
    None
}

fn reflect_smudge(grid: &Array2<bool>, axis: Axis) -> Option<usize> {
    let size = grid.len_of(axis);
    for pos in 1..size {
        let mut diffs = 0;
        let mut left = pos - 1;
        let mut right = pos;
        while diffs <= 1 && left < size && right < size {
            diffs += grid
                .index_axis(axis, left)
                .iter()
                .zip(grid.index_axis(axis, right).iter())
                .filter(|(left, right)| left != right)
                .count();

            left = left.wrapping_sub(1);
            right += 1;
        }
        if diffs == 1 {
            return Some(pos);
        }
    }
    None
}

pub struct Day13;
impl Day for Day13 {
    type Parsed = Vec<Array2<bool>>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .split("\n\n")
            .map(|grid| {
                parse_str_grid(grid, |c| c == '#').unwrap()
            })
            .collect())
    }
    fn first(grids: Self::Parsed) -> Self::Output {
        grids
            .into_iter()
            .map(|grid| {
                if let Some(score) = reflect(&grid, Axis(1)) {
                    score
                } else if let Some(score) = reflect(&grid, Axis(0)) {
                    score * 100
                } else {
                    0
                }
            })
            .sum()
    }
    fn second(grids: Self::Parsed) -> Self::Output {
        grids
            .into_iter()
            .map(|grid| {
                if let Some(score) = reflect_smudge(&grid, Axis(1)) {
                    score
                } else if let Some(score) = reflect_smudge(&grid, Axis(0)) {
                    score * 100
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    fn parsed() -> <Day13 as Day>::Parsed {
        Day13::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day13::first(parsed()), 405);
    }
    #[test]
    fn part2() {
        assert_eq!(Day13::second(parsed()), 400);
    }
}
