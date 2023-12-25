#![allow(clippy::cast_precision_loss)]
use crate::helpers::grid2d::Position2D;

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;

type PosI = Position2D<isize>;
type PosF = Position2D<f64>;
fn line_intersect_diff(a_start: PosI, a_diff: PosI, b_start: PosI, b_diff: PosI) -> Option<PosF> {
    let denominator = (b_diff.y * a_diff.x) - (b_diff.x * a_diff.y);
    if denominator == 0 {
        return None;
    }
    let numerator = (b_diff.x * (a_start.y - b_start.y)) - (b_diff.y * (a_start.x - b_start.x));
    let slope = numerator as f64 / denominator as f64;

    Some(PosF::new(
        slope.mul_add(a_diff.x as f64, a_start.x as f64),
        slope.mul_add(a_diff.y as f64, a_start.y as f64),
    ))
}

type Pos3D = (isize, isize, isize);

pub struct Day24;
impl Day for Day24 {
    type Parsed = Vec<(Pos3D, Pos3D)>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let (pos, vel) = line.split_once(" @ ").unwrap();
                let pos: Pos3D = pos
                    .split(", ")
                    .map(|n| n.trim().parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                let vel: Pos3D = vel
                    .split(", ")
                    .map(|n| n.trim().parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                (pos, vel)
            })
            .collect())
    }
    fn first(hail: Self::Parsed) -> Self::Output {
        xy_intersections(&hail, 200_000_000_000_000f64, 400_000_000_000_000f64)
    }
    fn second(_elves: Self::Parsed) -> Self::Output {
        0
    }
}

fn xy_intersections(hail: &[(Pos3D, Pos3D)], min: f64, max: f64) -> usize {
    let mut intersections = 0;
    for (a, b) in hail.iter().tuple_combinations() {
        let a_start = PosI::new(a.0 .0, a.0 .1);
        let a_diff = PosI::new(a.1 .0, a.1 .1);
        let b_start = PosI::new(b.0 .0, b.0 .1);
        let b_diff = PosI::new(b.1 .0, b.1 .1);

        if let Some(intersect) = line_intersect_diff(a_start, a_diff, b_start, b_diff) {
            if !(a_diff.x > 0 && intersect.x < a_start.x as f64
                || a_diff.x < 0 && intersect.x > a_start.x as f64
                || a_diff.y > 0 && intersect.y < a_start.y as f64
                || a_diff.y < 0 && intersect.y > a_start.y as f64
                || b_diff.x > 0 && intersect.x < b_start.x as f64
                || b_diff.x < 0 && intersect.x > b_start.x as f64
                || b_diff.y > 0 && intersect.y < b_start.y as f64
                || b_diff.y < 0 && intersect.y > b_start.y as f64
                || intersect.x < min
                || intersect.x > max
                || intersect.y < min
                || intersect.y > max)
            {
                intersections += 1;
            }
        }
    }
    intersections
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    fn parsed() -> <Day24 as Day>::Parsed {
        Day24::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(xy_intersections(&parsed(), 7f64, 27f64), 2);
    }
    #[test]
    fn part2() {
        //assert_eq!(Day24::second(parsed()), 45000);
    }
}
