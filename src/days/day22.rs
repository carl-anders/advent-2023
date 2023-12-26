use std::{
    collections::{HashMap, HashSet},
    mem,
};

use crate::helpers::grid2d::Position2D;

use super::day::Day;
use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Pos2D = Position2D<usize>;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos3D {
    x: usize,
    y: usize,
    z: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct Brick {
    left: Pos3D,
    right: Pos3D,
}
impl Brick {
    fn new(mut left: Pos3D, mut right: Pos3D) -> Self {
        if right.x < left.x || right.y < left.y || right.z < left.z {
            mem::swap(&mut left, &mut right);
        }
        Self { left, right }
    }
}

struct BrickLayer {
    len: usize,
    supported_by: HashMap<usize, HashSet<usize>>,
    supports: HashMap<usize, HashSet<usize>>,
    preserve: HashSet<usize>,
}
impl BrickLayer {
    fn compute(mut bricks: Vec<Brick>) -> Self {
        bricks.sort_unstable_by_key(|brick| brick.left.z);

        let mut pos_data: HashMap<Pos2D, (usize, usize)> = HashMap::new();
        let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();

        for (i, brick) in bricks.iter().enumerate() {
            let settle_at = (brick.left.x..=brick.right.x)
                .map(|x| {
                    (brick.left.y..=brick.right.y)
                        .map(|y| pos_data.get(&Pos2D::new(x, y)).map_or(0, |p| p.0))
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap();
            let brick_height = brick.right.z - brick.left.z + 1;

            for x in brick.left.x..=brick.right.x {
                for y in brick.left.y..=brick.right.y {
                    let data = pos_data
                        .entry(Pos2D::new(x, y))
                        .or_insert_with(|| (0, usize::MAX));
                    if data.1 != usize::MAX && data.0 == settle_at {
                        supported_by.entry(i).or_default().insert(data.1);
                        supports.entry(data.1).or_default().insert(i);
                    }
                    data.0 = settle_at + brick_height;
                    data.1 = i;
                }
            }
        }

        let mut preserve = HashSet::new();
        for support in supported_by.values() {
            if support.len() == 1 {
                let key = support.iter().next().unwrap();
                preserve.insert(*key);
            }
        }
        Self {
            len: bricks.len(),
            supported_by,
            supports,
            preserve,
        }
    }
    fn orphan_bricks(&self) -> usize {
        self.len - self.preserve.len()
    }
    fn total_cascades(&self) -> usize {
        (0..self.len)
            .into_par_iter()
            .filter(|origin| self.preserve.contains(origin))
            .map(|origin| {
                let mut support_map: HashMap<usize, usize> = HashMap::new();
                support_map.insert(origin, origin);
                self.deep_support(origin, &mut support_map, origin)
            })
            .sum()
    }
    fn deep_support(
        &self,
        current: usize,
        support_map: &mut HashMap<usize, usize>,
        origin: usize,
    ) -> usize {
        self.supports.get(&current).map_or(0, |supports| {
            supports
                .iter()
                .map(|&support| {
                    if support_map.get(&support).map_or(true, |&c| c != origin)
                        && self.supported_by[&support]
                            .iter()
                            .all(|support| support_map.get(support).is_some_and(|&c| c == origin))
                    {
                        support_map.insert(support, origin);
                        1 + self.deep_support(support, support_map, origin)
                    } else {
                        0
                    }
                })
                .sum()
        })
    }
}

pub struct Day22;
impl Day for Day22 {
    type Parsed = Vec<Brick>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let nums: Vec<usize> = line
                    .split(&[',', '~'])
                    .map(|n| n.parse().unwrap())
                    .collect();
                Brick::new(
                    Pos3D {
                        x: nums[0],
                        y: nums[1],
                        z: nums[2],
                    },
                    Pos3D {
                        x: nums[3],
                        y: nums[4],
                        z: nums[5],
                    },
                )
            })
            .collect())
    }
    fn first(bricks: Self::Parsed) -> Self::Output {
        BrickLayer::compute(bricks).orphan_bricks()
    }
    fn second(bricks: Self::Parsed) -> Self::Output {
        BrickLayer::compute(bricks).total_cascades()
    }
}

/*
fn print_sparse_3d_grid(grid: &HashMap<Pos3D, usize>, show_x: bool) {
    let x_minmax = grid
        .iter()
        .map(|(p, _)| p.x)
        .minmax()
        .into_option()
        .unwrap();
    let y_minmax = grid
        .iter()
        .map(|(p, _)| p.y)
        .minmax()
        .into_option()
        .unwrap();
    let z_minmax = grid
        .iter()
        .map(|(p, _)| p.z)
        .minmax()
        .into_option()
        .unwrap();
    if show_x {
        for z in (z_minmax.0..=z_minmax.1).rev() {
            for x in x_minmax.0..=x_minmax.1 {
                if grid.iter().any(|(p, _)| p.x == x && p.z == z) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    } else {
        for z in (z_minmax.0..=z_minmax.1).rev() {
            for y in y_minmax.0..=y_minmax.1 {
                if grid.iter().any(|(p, _)| p.y == y && p.z == z) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    fn parsed() -> <Day22 as Day>::Parsed {
        Day22::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day22::first(parsed()), 5);
    }
    #[test]
    fn part2() {
        assert_eq!(Day22::second(parsed()), 7);
    }
}
