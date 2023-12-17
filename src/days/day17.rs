use crate::helpers::grid2d::{parse_str_grid, Direction4Way, Position2D};

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;
use ndarray::Array2;
use pathfinding::directed::dijkstra;
use smallvec::SmallVec;

type Pos = Position2D<usize>;
type Dir = Direction4Way;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NormalPlanner {
    moves: SmallVec<[Dir; 3]>,
    pos: Pos,
}
impl NormalPlanner {
    fn new(pos: Pos) -> Self {
        Self {
            moves: SmallVec::new(),
            pos,
        }
    }
    fn paths(&self, grid: &Array2<u32>) -> SmallVec<[(Self, u32); 3]> {
        let mut new_directions: SmallVec<[Dir; 4]> = Dir::EVERY.into_iter().collect();
        if let Some(last_move) = self.moves.last() {
            let u_turn = last_move.turn_right(2);
            new_directions.retain(|dir| *dir != u_turn);
        }
        if self.moves.len() == 3 && self.moves[0] == self.moves[1] && self.moves[1] == self.moves[2]
        {
            new_directions.retain(|dir| *dir != self.moves[0]);
        }

        new_directions
            .iter()
            .filter_map(|dir| {
                let new_pos = self.pos + *dir;
                grid.get(new_pos.yx()).map(|&loss| {
                    let mut planner = self.clone();
                    if planner.moves.len() == 3 {
                        planner.moves.remove(0);
                    }
                    planner.moves.push(*dir);
                    planner.pos = new_pos;
                    (planner, loss)
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UltraPlanner {
    last_move: Option<Dir>,
    pos: Pos,
}
impl UltraPlanner {
    const fn new(pos: Pos) -> Self {
        Self {
            last_move: None,
            pos,
        }
    }
    fn paths(&self, grid: &Array2<u32>) -> SmallVec<[(Self, u32); 3]> {
        let mut new_directions: SmallVec<[Dir; 4]> = Dir::EVERY.into_iter().collect();
        if let Some(last_move) = self.last_move {
            let u_turn = last_move.turn_right(2);
            new_directions.retain(|dir| *dir != u_turn && *dir != last_move);
        }

        new_directions
            .into_iter()
            .cartesian_product(4..=10)
            .filter_map(|(dir, moves)| {
                let (new_pos, losses) = (0..moves).fold((self.pos, 0), |(old_pos, old_loss), _| {
                    let new_pos = old_pos + dir;
                    (new_pos, old_loss + *grid.get(new_pos.yx()).unwrap_or(&0))
                });
                grid.get(new_pos.yx()).map(|_| {
                    (
                        Self {
                            pos: new_pos,
                            last_move: Some(dir),
                        },
                        losses,
                    )
                })
            })
            .collect()
    }
}

pub struct Day17;
impl Day for Day17 {
    type Parsed = Array2<u32>;
    type Output = u32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(parse_str_grid(&input, |c| c.to_digit(10).unwrap())?)
    }
    fn first(grid: Self::Parsed) -> Self::Output {
        let end_pos = Pos::new_yx((grid.shape()[0] - 1, grid.shape()[1] - 1));
        dijkstra::dijkstra(
            &NormalPlanner::new(Pos::new(0, 0)),
            |path_planner| path_planner.paths(&grid),
            |path_planner| path_planner.pos == end_pos,
        )
        .unwrap()
        .1
    }
    fn second(grid: Self::Parsed) -> Self::Output {
        let end_pos = Pos::new_yx((grid.shape()[0] - 1, grid.shape()[1] - 1));
        dijkstra::dijkstra(
            &UltraPlanner::new(Pos::new(0, 0)),
            |path_planner| path_planner.paths(&grid),
            |path_planner| path_planner.pos == end_pos,
        )
        .unwrap()
        .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    fn parsed() -> <Day17 as Day>::Parsed {
        Day17::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day17::first(parsed()), 102);
    }
    #[test]
    fn part2() {
        assert_eq!(Day17::second(parsed()), 94);
    }
}
