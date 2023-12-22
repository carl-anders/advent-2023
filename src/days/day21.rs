#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
use polyfit_rs::polyfit_rs::polyfit;
use std::collections::HashSet;

use crate::helpers::grid2d::{parse_str_grid, Direction4Way, Position2D};

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ground {
    Garden,
    Rock,
}

type Pos = Position2D<isize>;
type Dir = Direction4Way;

#[allow(dead_code)]
fn print_sparse_grid(grid: &HashSet<Pos>) {
    let ys = grid.iter().map(|p| p.y).minmax().into_option().unwrap();
    let xs = grid.iter().map(|p| p.x).minmax().into_option().unwrap();
    for y in ys.0..=ys.1 {
        for x in xs.0..=xs.1 {
            if grid.contains(&Pos::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn visited_gardens(start: Pos, grid: &Array2<Ground>, steps: &[usize]) -> Vec<usize> {
    let [height, width] = *grid.shape() else {
        panic!()
    };
    let (height, width) = (height as isize, width as isize);

    let max_steps = *steps.iter().max().unwrap();
    let mut results = vec![0; steps.len()];

    let mut visited: [HashSet<Pos>; 2] = [HashSet::new(), HashSet::new()];
    let mut current: Vec<Pos> = vec![start];
    for curr_step in 0..max_steps {
        let index = curr_step % 2;
        let mut new_pos = vec![];
        for pos in &current {
            for dir in Dir::EVERY {
                let pos_dir = *pos + dir;
                let mod_pos = Position2D::new(
                    pos_dir.x.rem_euclid(width) as usize,
                    pos_dir.y.rem_euclid(height) as usize,
                );
                if !visited[index].contains(&pos_dir) && grid[mod_pos.yx()] == Ground::Garden {
                    new_pos.push(pos_dir);
                    visited[index].insert(pos_dir);
                }
            }
        }
        current = new_pos;
        if let Some(save) = steps.iter().position(|&ns| ns == curr_step + 1) {
            results[save] = visited[curr_step % 2].len();
        }
    }
    results
}

pub struct Day21;
impl Day for Day21 {
    type Parsed = (Pos, Array2<Ground>);
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let start = Pos::new_yx(
            input
                .lines()
                .enumerate()
                .find_map(|(y, line)| {
                    line.chars()
                        .position(|c| c == 'S')
                        .map(|x| (y as isize, x as isize))
                })
                .unwrap(),
        );
        let grid = parse_str_grid(&input, |c| match c {
            '.' | 'S' => Ground::Garden,
            '#' => Ground::Rock,
            _ => panic!(),
        })?;
        Ok((start, grid))
    }
    fn first((start_pos, grid): Self::Parsed) -> Self::Output {
        visited_gardens(start_pos, &grid, &[64])[0]
    }
    fn second((start_pos, grid): Self::Parsed) -> Self::Output {
        let [height, width] = *grid.shape() else {
            panic!()
        };
        assert!(height == width, "Part 2 only supports square inputs");

        // Estimation of visited gardens based on cycle of input grid size.
        // This is accurate for aoc input data, but not general.

        let total_steps = 26_501_365;
        let cycles = total_steps / width;
        let cycle_start = total_steps % width;

        let steps: Vec<usize> = (0..3).map(|i| cycle_start + i * width).collect();
        let points: Vec<f64> = visited_gardens(start_pos, &grid, &steps)
            .into_iter()
            .map(|v| v as f64)
            .collect();

        let p_fit: Vec<usize> = polyfit(&[0f64, 1f64, 2f64], &points, 2)
            .unwrap()
            .into_iter()
            .map(|v| v.round() as usize)
            .collect();

        p_fit[2] * cycles * cycles + p_fit[1] * cycles + p_fit[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    fn parsed() -> <Day21 as Day>::Parsed {
        Day21::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day21::first(parsed()), 2665);
    }
    #[test]
    fn example_garden_list() {
        let (start_pos, grid) = parsed();
        assert_eq!(
            visited_gardens(start_pos, &grid, &[6, 10, 50, 100]),
            [16, 50, 1594, 6536]
        );
    }
}
