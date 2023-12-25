use std::collections::{HashMap, HashSet, VecDeque};

use crate::helpers::grid2d::{parse_str_grid, Direction4Way, Position2D};

use super::day::Day;
use anyhow::Result;
use ndarray::Array2;
use smallvec::{smallvec, SmallVec};

type Dir = Direction4Way;
type Pos = Position2D<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spot {
    Path,
    Forest,
    Slope(Dir),
}

pub struct Day23;
impl Day for Day23 {
    type Parsed = (Array2<Spot>, Pos, Pos);
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let grid = parse_str_grid(&input, |c| match c {
            '.' => Spot::Path,
            '#' => Spot::Forest,
            '>' => Spot::Slope(Dir::East),
            'v' => Spot::Slope(Dir::South),
            '<' => Spot::Slope(Dir::West),
            '^' => Spot::Slope(Dir::North),
            _ => panic!(),
        })?;
        let start = Pos::new(input.lines().next().unwrap().find('.').unwrap(), 0);
        let end = Pos::new(
            input.lines().next_back().unwrap().find('.').unwrap(),
            grid.shape()[0] - 1,
        );
        Ok((grid, start, end))
    }
    fn first((grid, start, end): Self::Parsed) -> Self::Output {
        let [height, width] = *grid.shape() else {
            panic!()
        };

        let mut max_length = 0;
        let mut queue: VecDeque<(Pos, HashSet<Pos>)> = VecDeque::new();
        queue.push_back((start, HashSet::new()));
        while let Some((curr, visited)) = queue.pop_front() {
            for dir in Dir::EVERY {
                let next = curr + dir;
                if next.x < width && next.y < height && !visited.contains(&next) {
                    if next == end {
                        max_length = max_length.max(visited.len());
                    } else {
                        let spot = grid[next.yx()];
                        if match spot {
                            Spot::Path => true,
                            Spot::Forest => false,
                            Spot::Slope(slope_dir) => slope_dir == dir,
                        } {
                            let mut new_visit = visited.clone();
                            new_visit.insert(next);
                            queue.push_back((next, new_visit));
                        }
                    }
                }
            }
        }
        max_length + 1
    }
    fn second((grid, start, end): Self::Parsed) -> Self::Output {
        let [height, width] = *grid.shape() else {
            panic!()
        };

        let mut grid_nodes: HashMap<Pos, Node> = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                let pos = Pos::new(x, y);
                if grid.get(pos.yx()).is_some_and(|g| *g == Spot::Forest) {
                    continue;
                }

                let ways = get_ways(&grid, pos);
                if pos == start || pos == end || ways.len() > 2 {
                    grid_nodes.insert(
                        pos,
                        Node {
                            connections: smallvec![],
                        },
                    );
                    for mut way in ways {
                        let mut last = pos;
                        let mut steps = 1;
                        loop {
                            let new_ways = get_ways(&grid, way);
                            if way == start || way == end || new_ways.len() > 2 {
                                grid_nodes.get_mut(&pos).unwrap().connections.push(Edge {
                                    to: way,
                                    length: steps,
                                });
                                break;
                            }

                            steps += 1;
                            let current_way = way;
                            way = *new_ways.iter().find(|p| **p != last).unwrap();
                            last = current_way;
                        }
                    }
                }
            }
        }

        let mut max_steps = 0;
        let mut queue: VecDeque<(Pos, HashSet<Pos>, usize)> = VecDeque::new();
        queue.push_back((start, HashSet::new(), 0));
        while let Some((curr, visited, steps)) = queue.pop_front() {
            for conn in &grid_nodes[&curr].connections {
                if conn.to == end {
                    max_steps = max_steps.max(steps + conn.length);
                } else if !visited.contains(&conn.to) {
                    let mut new_visit = visited.clone();
                    new_visit.insert(conn.to);
                    queue.push_back((conn.to, new_visit, steps + conn.length));
                }
            }
        }
        max_steps
    }
}

fn get_ways(grid: &Array2<Spot>, pos: Pos) -> SmallVec<[Pos; 4]> {
    Dir::EVERY
        .into_iter()
        .filter_map(|d| {
            grid.get((pos + d).yx()).and_then(|g| {
                if *g == Spot::Forest {
                    None
                } else {
                    Some(pos + d)
                }
            })
        })
        .collect()
}
#[derive(Clone, Debug)]
struct Node {
    connections: SmallVec<[Edge; 4]>,
}
#[derive(Clone, Debug)]
struct Edge {
    to: Pos,
    length: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    fn parsed() -> <Day23 as Day>::Parsed {
        Day23::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day23::first(parsed()), 94);
    }
    #[test]
    fn part2() {
        assert_eq!(Day23::second(parsed()), 154);
    }
}
