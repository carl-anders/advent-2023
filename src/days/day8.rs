use std::collections::HashMap;

use crate::helpers::grid2d::Turn;

use super::day::Day;
use anyhow::Result;
use num_integer::Integer;

#[derive(Debug, Clone)]
pub struct Input {
    path: Vec<Turn>,
    nodes: HashMap<String, (String, String)>
}
pub struct Day8;
impl Day for Day8 {
    type Parsed = Input;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let (path, node_list) = input.split_once("\n\n").unwrap();
        let path = path
            .chars()
            .map(|c| if c == 'R' { Turn::Right } else { Turn::Left })
            .collect();

        let mut nodes: HashMap<String, (String, String)> = HashMap::new();

        for line in node_list.lines() {
            let (node, paths) = line.split_once(" = ").unwrap();
            let (left, right) = paths.split_once(", ").unwrap();
            nodes.insert(
                node.to_string(),
                (left[1..4].to_string(), right[0..3].to_string()),
            );
        }

        Ok(Input { path, nodes })
    }
    fn first(input: Self::Parsed) -> Self::Output {
        let mut position = "AAA".to_string();
        for (steps, turn) in input.path.iter().cycle().enumerate() {
            if position == "ZZZ" {
                return steps;
            }
            if *turn == Turn::Left {
                position = input.nodes.get(&position).unwrap().0.clone();
            } else {
                position = input.nodes.get(&position).unwrap().1.clone();
            }
        }
        unreachable!()
    }
    fn second(input: Self::Parsed) -> Self::Output {
        let mut positions: Vec<(String, usize)> = input.nodes
            .keys()
            .filter(|node| node.ends_with('A'))
            .cloned()
            .map(|s| (s, 0))
            .collect();
        for (position, steps) in &mut positions {
            for turn in input.path.iter().cycle() {
                if position.ends_with('Z') {
                    break;
                }
                *steps += 1;
                if *turn == Turn::Left {
                    *position = input.nodes.get(position).unwrap().0.clone();
                } else {
                    *position = input.nodes.get(position).unwrap().1.clone();
                }
            }
        }
        positions.iter().fold(1, |acc, (_, steps)| acc.lcm(steps))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    fn parsed() -> <Day8 as Day>::Parsed {
        Day8::parse(INPUT.to_string()).unwrap()
    }
    fn parsed2() -> <Day8 as Day>::Parsed {
        Day8::parse(INPUT2.to_string()).unwrap()
    }
    fn parsed3() -> <Day8 as Day>::Parsed {
        Day8::parse(INPUT3.to_string()).unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(Day8::first(parsed()), 2);
        assert_eq!(Day8::first(parsed2()), 6);
    }
    #[test]
    fn part2() {
        assert_eq!(Day8::second(parsed3()), 6);
    }
}
