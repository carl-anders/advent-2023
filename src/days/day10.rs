use std::collections::HashSet;

use crate::helpers::grid2d::{Direction4Way, Position2D, parse_str_grid};

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;
use ndarray::Array2;

type Pos = Position2D<usize>;
type Dir = Direction4Way;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}
impl Pipe {
    fn from_char(c: char) -> Self {
        use Pipe::*;
        match c {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => Ground,
            'S' => Start,
            _ => panic!(),
        }
    }
    const fn format(self) -> char {
        use Pipe::*;
        match self {
            Vertical => '│',
            Horizontal => '─',
            NE => '└',
            NW => '┘',
            SW => '┐',
            SE => '┌',
            Ground => ' ',
            Start => 'S',
        }
    }
    const fn connects_to(self, dir: Dir) -> bool {
        match dir {
            Dir::East => {
                matches!(self, Self::Horizontal | Self::NE | Self::SE)
            }
            Dir::South => {
                matches!(self, Self::Vertical | Self::SE | Self::SW)
            }
            Dir::West => {
                matches!(self, Self::Horizontal | Self::NW | Self::SW)
            }
            Dir::North => {
                matches!(self, Self::Vertical | Self::NE | Self::NW)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Maze {
    maze: Array2<Pipe>,
    start: Pos,
}
impl Maze {
    #[allow(dead_code)]
    fn format(&self) -> String {
        use std::fmt::Write;
        let mut output = String::new();
        for line in self.maze.outer_iter() {
            for pipe in line {
                let _ = write!(output, "{}", pipe.format());
            }
            let _ = writeln!(output);
        }
        output
    }
    fn fix_start(&mut self) {
        let pipes: (bool, bool, bool, bool) = Dir::EVERY
            .iter()
            .map(|dir| {
                self.maze
                    .get((self.start + *dir).yx())
                    .map_or(false, |pipe| pipe.connects_to(dir.turn_right(2)))
            })
            .collect_tuple()
            .unwrap();
        // North, East, South, West
        let start_pipe = match pipes {
            (true, true, false, false) => Pipe::NE,
            (true, false, false, true) => Pipe::NW,
            (false, false, true, true) => Pipe::SW,
            (false, true, true, false) => Pipe::SE,
            (true, false, true, false) => Pipe::Vertical,
            (false, true, false, true) => Pipe::Horizontal,
            _ => panic!("Invalid start pipe"),
        };

        *self.maze.get_mut(self.start.yx()).unwrap() = start_pipe;
    }
    fn walk(&self, visited: &HashSet<Pos>, from: Pos) -> Option<Pos> {
        let pipe = self.maze.get(from.yx()).unwrap();
        Dir::EVERY
            .iter()
            .filter(|&&dir| pipe.connects_to(dir))
            .map(|&dir| from + dir)
            .find(|pos| !visited.contains(pos))
    }
    fn step_through(&self) -> (usize, HashSet<Pos>) {
        let pipe = self.maze.get(self.start.yx()).unwrap();
        let mut positions: [Pos; 2] = Dir::EVERY
            .iter()
            .filter(|&&dir| pipe.connects_to(dir))
            .map(|&dir| self.start + dir)
            .collect::<Vec<Pos>>()
            .try_into()
            .unwrap();

        let mut visited = HashSet::from([self.start, positions[0], positions[1]]);
        let mut steps = 1;

        loop {
            steps += 1;
            for position in &mut positions {
                if let Some(new_position) = self.walk(&visited, *position) {
                    *position = new_position;
                    visited.insert(new_position);
                } else {
                    return (steps, visited);
                }
            }
        }
    }
    fn max_steps(&self) -> usize {
        self.step_through().0
    }
    fn count_inner_spaces(&self) -> usize {
        let visited = self.step_through().1;
        self.maze
            .outer_iter()
            .enumerate()
            .map(|(y, line)| {
                let mut is_in = false;
                line.iter()
                    .enumerate()
                    .filter(|&(x, pipe)| {
                        let pos = Pos::new(x, y);
                        if visited.contains(&pos) {
                            match pipe {
                                Pipe::Vertical | Pipe::NE | Pipe::NW => {
                                    is_in = !is_in;
                                }
                                _ => {}
                            }
                            false
                        } else {
                            is_in
                        }
                    })
                    .count()
            })
            .sum()
    }
}

pub struct Day10;
impl Day for Day10 {
    type Parsed = Maze;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let maze = parse_str_grid(&input, Pipe::from_char)?;
        let start = Pos::new_yx(
            maze.indexed_iter()
                .find(|(_, pipe)| **pipe == Pipe::Start)
                .unwrap()
                .0,
        );
        Ok(Maze { maze, start })
    }
    fn first(mut maze: Self::Parsed) -> Self::Output {
        maze.fix_start();
        maze.max_steps()
    }
    fn second(mut maze: Self::Parsed) -> Self::Output {
        maze.fix_start();
        maze.count_inner_spaces()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const INPUT2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    const INPUT3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const INPUT4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    const INPUT5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    fn parsed() -> <Day10 as Day>::Parsed {
        Day10::parse(INPUT.to_string()).unwrap()
    }
    fn parsed2() -> <Day10 as Day>::Parsed {
        Day10::parse(INPUT2.to_string()).unwrap()
    }
    fn parsed3() -> <Day10 as Day>::Parsed {
        Day10::parse(INPUT3.to_string()).unwrap()
    }
    fn parsed4() -> <Day10 as Day>::Parsed {
        Day10::parse(INPUT4.to_string()).unwrap()
    }
    fn parsed5() -> <Day10 as Day>::Parsed {
        Day10::parse(INPUT5.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day10::first(parsed()), 4);
        assert_eq!(Day10::first(parsed2()), 8);
    }
    #[test]
    fn part2() {
        assert_eq!(Day10::second(parsed3()), 4);
        assert_eq!(Day10::second(parsed4()), 8);
        assert_eq!(Day10::second(parsed5()), 10);
    }
}
