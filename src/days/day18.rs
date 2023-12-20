#![allow(clippy::cast_possible_wrap)]
use crate::helpers::grid2d::{Direction4Way, Position2D};

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;

type Dir = Direction4Way;
type Pos = Position2D<isize>;
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    steps: isize,
    dir: Dir,
}

fn pos_add(pos: Pos, dir: Dir, steps: isize) -> Pos {
    match dir {
        Dir::East => pos.wrapping_add_x(&steps),
        Dir::South => pos.wrapping_add_y(&steps),
        Dir::West => pos.wrapping_sub_x(&steps),
        Dir::North => pos.wrapping_sub_y(&steps),
    }
}

fn path_area(instructions: &Vec<Instruction>) -> isize {
    let mut pos = Pos::new(0, 0);

    let mut path = vec![pos];
    let mut distance = 0;
    for instruction in instructions {
        distance += instruction.steps;
        pos = pos_add(pos, instruction.dir, instruction.steps);
        path.push(pos);
    }

    let area = path
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<isize>();

    ((area.abs() + distance) / 2) + 1
}

pub struct Day18;
impl Day for Day18 {
    type Parsed = (Vec<Instruction>, Vec<Instruction>);
    type Output = isize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let (dir, rest) = line.split_once(' ').unwrap();
                let (steps, color) = rest.split_once(' ').unwrap();

                let steps = steps.parse().unwrap();
                let dir = match dir.chars().next().unwrap() {
                    'U' => Dir::North,
                    'D' => Dir::South,
                    'R' => Dir::East,
                    'L' => Dir::West,
                    _ => panic!(),
                };
                let p1 = Instruction { steps, dir };

                let steps = isize::from_str_radix(&color[2..7], 16).unwrap();
                let dir = match color[7..].chars().next().unwrap() {
                    '0' => Dir::East,
                    '1' => Dir::South,
                    '2' => Dir::West,
                    '3' => Dir::North,
                    _ => panic!(),
                };
                let p2 = Instruction { steps, dir };

                (p1, p2)
            })
            .unzip())
    }
    fn first((instructions, _): Self::Parsed) -> Self::Output {
        path_area(&instructions)
    }
    fn second((_, instructions): Self::Parsed) -> Self::Output {
        path_area(&instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    fn parsed() -> <Day18 as Day>::Parsed {
        Day18::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day18::first(parsed()), 62);
    }
    #[test]
    fn part2() {
        assert_eq!(Day18::second(parsed()), 952408144115);
    }
}
