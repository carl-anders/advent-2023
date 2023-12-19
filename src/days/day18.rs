use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use crate::helpers::grid2d::{Direction4Way, Position2D};

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;

type Dir = Direction4Way;
type Pos = Position2D<isize>;
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    dir: Dir,
    steps: usize,
}

fn print_pos_map(map: &HashSet<Pos>) {
    let limits = map_limits(map);
    for y in limits.1 .0..=limits.1 .1 {
        for x in limits.0 .0..=limits.0 .1 {
            if map.contains(&Pos::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn map_limits(map: &HashSet<Pos>) -> ((isize, isize), (isize, isize)) {
    let (x_min, x_max) = map.iter().map(|pos| pos.x).minmax().into_option().unwrap();
    let (y_min, y_max) = map.iter().map(|pos| pos.y).minmax().into_option().unwrap();
    ((x_min, x_max), (y_min, y_max))
}

fn pos_add_many_dir(pos: &Pos, dir: &Dir, count: isize) -> Pos {
    match dir {
        Direction4Way::East => pos.wrapping_add_x(&count),
        Direction4Way::South => pos.wrapping_add_y(&count),
        Direction4Way::West => pos.wrapping_sub_x(&count),
        Direction4Way::North => pos.wrapping_sub_y(&count),
    }
}

fn visit(
    map: &mut HashSet<Pos>,
    terrain: &HashSet<Pos>,
    x_limit: &RangeInclusive<isize>,
    y_limit: &RangeInclusive<isize>,
    pos: Pos,
) {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(pos);
    while let Some(visit) = to_visit.pop_front() {
        for dir in Dir::EVERY {
            let new_pos = visit + dir;
            if x_limit.contains(&new_pos.x)
                && y_limit.contains(&new_pos.y)
                && !map.contains(&new_pos)
                && !terrain.contains(&new_pos)
            {
                map.insert(new_pos);
                to_visit.push_back(new_pos);
            }
        }
    }
}

pub struct Day18;
impl Day for Day18 {
    type Parsed = (Vec<Instruction>, Vec<Instruction>);
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let p1 = input
            .lines()
            .map(|line| {
                let (dir, rest) = line.split_once(' ').unwrap();
                let (steps, _) = rest.split_once(' ').unwrap();
                let dir = match dir.chars().next().unwrap() {
                    'U' => Dir::North,
                    'D' => Dir::South,
                    'R' => Dir::East,
                    'L' => Dir::West,
                    _ => panic!(),
                };
                let steps = steps.parse().unwrap();
                Instruction { dir, steps }
            })
            .collect();
        let p2 = input
            .lines()
            .map(|line| {
                let (_, color) = line.rsplit_once(' ').unwrap();

                let steps = usize::from_str_radix(&color[2..7], 16).unwrap();
                let dir = match color[7..].chars().next().unwrap() {
                    '0' => Dir::East,
                    '1' => Dir::South,
                    '2' => Dir::West,
                    '3' => Dir::North,
                    _ => panic!(),
                };

                Instruction { dir, steps }
            })
            .collect();
        Ok((p1, p2))
    }
    fn first((instructions, _): Self::Parsed) -> Self::Output {
        let mut position = Pos::new(0, 0);
        let mut terrain = HashSet::new();
        terrain.insert(position);
        for instruction in instructions {
            for _ in 0..instruction.steps {
                position = position + instruction.dir;
                terrain.insert(position);
            }
        }

        let limits = map_limits(&terrain);
        let x_range = (limits.0 .0 - 1)..=(limits.0 .1 + 1);
        let y_range = (limits.1 .0 - 1)..=(limits.1 .1 + 1);
        let mut outside = HashSet::new();
        visit(
            &mut outside,
            &terrain,
            &x_range,
            &y_range,
            Pos::new(limits.0 .0 - 1, limits.1 .0 - 1),
        );

        let mut lava = 0;
        for y in limits.1 .0..=limits.1 .1 {
            for x in limits.0 .0..=limits.0 .1 {
                let pos = Pos::new(x, y);
                if !outside.contains(&pos) {
                    lava += 1;
                }
            }
        }

        lava
    }
    fn second((_, instructions): Self::Parsed) -> Self::Output {
        let mut position = Pos::new(0, 0);
        let mut path = Vec::new();
        path.push(position);
        let mut distance = 0;
        for instruction in &instructions {
            position = pos_add_many_dir(&position, &instruction.dir, instruction.steps as isize);
            path.push(position);
            distance += instruction.steps;
        }
        let area = path
            .iter()
            .tuple_windows()
            .map(|(a, b)| a.x * b.y - a.y * b.x)
            .sum::<isize>();
        let area = ((area.abs() + distance as isize) / 2) + 1;

        return area as usize;

        /*
        let x_values: Vec<_> = path.iter().map(|pos| pos.x).unique().sorted().collect();
        let y_values: Vec<_> = path.iter().map(|pos| pos.y).unique().sorted().collect();
        let x_map: HashMap<_, _> = x_values.iter().enumerate().map(|(i, val)| (*val, i*3)).collect();
        let y_map: HashMap<_, _> = y_values.iter().enumerate().map(|(i, val)| (*val, i*3)).collect();
        let x_rev_map: HashMap<_, _> = x_values.iter().enumerate().map(|(i, val)| (i*3, *val)).collect();
        let y_rev_map: HashMap<_, _> = y_values.iter().enumerate().map(|(i, val)| (i*3, *val)).collect();

        let mut terrain = HashSet::new();
        let mut real_position = Pos::new(0, 0);
        let mut fake_position = Pos::new(x_map[&0] as isize, y_map[&0] as isize);
        terrain.insert(position);
        for instruction in &instructions {
            //println!("Instruction: {instruction:?}");
            let new_position = pos_add_many_dir(&real_position, &instruction.dir, instruction.steps as isize);
            //println!("New position: {new_position:?}");
            match instruction.dir {
                Dir::East => {
                    let from = x_map[&real_position.x];
                    let to = x_map[&new_position.x];
                    //println!("East. From: {from}. To: {to}");
                    for x in from..to {
                        fake_position = fake_position + instruction.dir;
                        //println!("  {x}: {fake_position:?}");
                        terrain.insert(fake_position);
                    }
                },
                Dir::South => {
                    let from = y_map[&real_position.y];
                    let to = y_map[&new_position.y];
                    //println!("South. From: {from}. To: {to}");
                    for y in from..to {
                        fake_position = fake_position + instruction.dir;
                        //println!("  {y}: {fake_position:?}");
                        terrain.insert(fake_position);
                    }
                },
                Dir::West => {
                    let from = x_map[&real_position.x];
                    let to = x_map[&new_position.x];
                    //println!("West. From: {from}. To: {to}");
                    for x in (to..from).rev() {
                        fake_position = fake_position + instruction.dir;
                        //println!("  {x}: {fake_position:?}");
                        terrain.insert(fake_position);
                    }
                },
                Dir::North => {
                    let from = y_map[&real_position.y];
                    let to = y_map[&new_position.y];
                    //println!("North. From: {from}. To: {to}");
                    for y in (to..from).rev() {
                        fake_position = fake_position + instruction.dir;
                        //println!("  {y}: {fake_position:?}");
                        terrain.insert(fake_position);
                    }
                }
            }
            real_position = new_position;
        }

        //dbg!(&terrain);

        let limits = map_limits(&terrain);
        let x_range = (limits.0 .0 - 1)..=(limits.0 .1 + 1);
        let y_range = (limits.1 .0 - 1)..=(limits.1 .1 + 1);
        dbg!(&y_range);
        dbg!(&y_rev_map);
        let mut outside = HashSet::new();
        visit(
            &mut outside,
            &terrain,
            &x_range,
            &y_range,
            Pos::new(limits.0 .0 - 1, limits.1 .0 - 1),
        );

        let mut lava = 0;
        for y in limits.1.0..=limits.1.1 {
            for x in limits.0.0..=limits.0.1 {
                let pos = Pos::new(x, y);
                if x % 3 == 0 && y % 3 == 0 && !outside.contains(&pos) {
                    if x/3 == 3 && y/3 == 2 {
                    println!("Coordinate inside: {pos:?}");
                    }
                    let left_x = x_rev_map[&(x as usize)];
                    if x/3 == 3 && y/3 == 2 {
                    println!("Real left x: {left_x}");
                    }
                    let left_y = y_rev_map[&(y as usize)];
                    if x/3 == 3 && y/3 == 2 {
                    println!("Real left y: {left_y}");
                    }
                    let width = if outside.contains(&(pos + Dir::East)) {
                        1
                    } else {
                        if let Some(right_x) = x_rev_map.get(&((x+3) as usize)) {
                            //println!("Real right x: {right_x}");
                            right_x - left_x
                        } else {
                            1
                        }
                    };
                    let height = if outside.contains(&(pos + Dir::South)) {
                        1
                    } else {
                        if let Some(right_y) = y_rev_map.get(&((y+3) as usize)) {
                            //println!("Real right y: {right_y}");
                            right_y - left_y
                        } else {
                            1
                        }
                    };
                    if x/3 == 3 && y/3 == 2 {
                    println!("x: {}, y: {}, width: {}, height: {}", x/3, y/3, width, height);
                    }
                    //println!("Width: {width}, height: {height}, sum: {}", width*height);
                    lava += width*height;
                }
            }
        } */
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
