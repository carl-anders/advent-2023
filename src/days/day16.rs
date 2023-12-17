use crate::helpers::{
    grid2d::{parse_str_grid, Direction4Way, Position2D},
    BitArray,
};

use super::day::Day;
use anyhow::Result;
use ndarray::Array2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use smallvec::{smallvec, SmallVec};

type Dir = Direction4Way;
type Pos = Position2D<usize>;

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    MirrorRight, // /
    MirrorLeft,  // \
    SplitVert,   // |
    SplitHori,   // -
}
impl Tile {
    const fn from_char(c: char) -> Self {
        match c {
            '/' => Self::MirrorRight,
            '\\' => Self::MirrorLeft,
            '|' => Self::SplitVert,
            '-' => Self::SplitHori,
            _ => Self::Empty,
        }
    }
    const fn travel(self, dir: Dir) -> (Dir, Option<Dir>) {
        match (self, dir) {
            (Self::MirrorRight, Dir::East) | (Self::MirrorLeft, Dir::West) => (Dir::North, None),
            (Self::MirrorRight, Dir::South) | (Self::MirrorLeft, Dir::North) => (Dir::West, None),
            (Self::MirrorRight, Dir::West) | (Self::MirrorLeft, Dir::East) => (Dir::South, None),
            (Self::MirrorRight, Dir::North) | (Self::MirrorLeft, Dir::South) => (Dir::East, None),
            (Self::SplitVert, Dir::East | Dir::West) => (Dir::North, Some(Dir::South)),
            (Self::SplitHori, Dir::South | Dir::North) => (Dir::East, Some(Dir::West)),
            _ => (dir, None),
        }
    }
}

struct Lazer {
    pos: Pos,
    dir: Dir,
}
impl Lazer {
    const fn new(pos: Pos, dir: Dir) -> Self {
        Self { pos, dir }
    }
    fn simulate_lazer(tiles: &Array2<Tile>, start: Self) -> usize {
        let mut visited: Array2<u8> = Array2::default(tiles.raw_dim());
        let mut lazers: SmallVec<[_; 64]> = smallvec![start];
        while let Some(mut lazer) = lazers.pop() {
            while let Some(tile) = tiles.get(lazer.pos.yx()) {
                if visited[lazer.pos.yx()].get(lazer.dir as u8) {
                    break;
                }
                visited[lazer.pos.yx()].set(lazer.dir as u8);
                let travels = tile.travel(lazer.dir);
                if let Some(second) = travels.1 {
                    visited[lazer.pos.yx()].set(lazer.dir.turn_right(2) as u8);
                    lazers.push(Self::new(lazer.pos + second, second));
                }
                lazer.pos = lazer.pos + travels.0;
                lazer.dir = travels.0;
            }
        }

        visited
            .into_iter()
            .map(|visit| usize::from(visit != 0))
            .sum()
    }
}

pub struct Day16;
impl Day for Day16 {
    type Parsed = Array2<Tile>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(parse_str_grid(&input, Tile::from_char)?)
    }
    fn first(tiles: Self::Parsed) -> Self::Output {
        Lazer::simulate_lazer(&tiles, Lazer::new(Pos::new(0, 0), Dir::East))
    }
    fn second(tiles: Self::Parsed) -> Self::Output {
        let [height, width] = *tiles.shape() else {
            panic!()
        };
        (0..height)
            .into_par_iter()
            .flat_map(|y| {
                [
                    Lazer::new(Pos::new(0, y), Dir::East),
                    Lazer::new(Pos::new(width - 1, y), Dir::West),
                ]
            })
            .chain((0..width).into_par_iter().flat_map(|x| {
                [
                    Lazer::new(Pos::new(x, 0), Dir::South),
                    Lazer::new(Pos::new(x, height - 1), Dir::North),
                ]
            }))
            .map(|lazer| Lazer::simulate_lazer(&tiles, lazer))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    fn parsed() -> <Day16 as Day>::Parsed {
        Day16::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day16::first(parsed()), 46);
    }
    #[test]
    fn part2() {
        assert_eq!(Day16::second(parsed()), 51);
    }
}
