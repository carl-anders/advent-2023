use std::{cmp::Ordering, collections::HashMap};

use super::day::Day;
use anyhow::Result;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Clone, Copy)]
pub enum CompType {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}
impl CompType {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Rule {
    Straight {
        to: String,
    },
    Comparison {
        ct: CompType,
        comparison: Ordering,
        value: usize,
        to: String,
    },
}

#[derive(Debug, Clone)]
pub struct Workflow(Vec<Rule>);
impl Workflow {
    fn filter(&self, part: &Part) -> String {
        for rule in &self.0 {
            match rule {
                Rule::Straight { to } => return to.clone(),
                Rule::Comparison {
                    ct,
                    comparison,
                    value,
                    to,
                } => {
                    let part_val = part[*ct as usize];
                    if part_val.cmp(value) == *comparison {
                        return to.clone();
                    }
                }
            }
        }
        panic!();
    }
    fn ranges(&self, mut part: PartRange) -> SmallVec<[(String, PartRange); 4]> {
        let mut ranges = smallvec![];
        for rule in &self.0 {
            match rule {
                Rule::Straight { to } => ranges.push((to.clone(), part)),
                Rule::Comparison {
                    ct,
                    comparison,
                    value,
                    to,
                } => {
                    let index = *ct as usize;
                    let range = part[index];
                    match comparison {
                        Ordering::Less => {
                            if range.0 < *value {
                                if range.1 - 1 < *value {
                                    // Full range matches comparison
                                    ranges.push((to.clone(), part));
                                    return ranges;
                                }
                                let mut left = part;
                                left[index] = (range.0, *value);
                                ranges.push((to.clone(), left));
                                part[index] = (*value, range.1);
                            }
                        }
                        Ordering::Greater => {
                            if range.1 - 1 > *value {
                                if range.0 > *value {
                                    // Full range matches comparison
                                    ranges.push((to.clone(), part));
                                    return ranges;
                                }
                                let mut right = part;
                                let value = *value + 1;
                                right[index] = (value, range.1);
                                ranges.push((to.clone(), right));
                                part[index] = (range.0, value);
                            }
                        }
                        Ordering::Equal => panic!(),
                    }
                }
            }
        }
        ranges
    }
}

type Part = [usize; 4];
type PartRange = [(usize, usize); 4];

pub struct Day19;
impl Day for Day19 {
    type Parsed = (HashMap<String, Workflow>, Vec<Part>);
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        let (workflows, parts) = input.split_once("\n\n").unwrap();
        let workflows = workflows
            .lines()
            .map(|line| {
                let (id, rules) = line.split_once('{').unwrap();
                let rules = rules[..rules.len() - 1]
                    .split(',')
                    .map(|rule| {
                        if let Some((left, to)) = rule.split_once(':') {
                            Rule::Comparison {
                                ct: CompType::from_char(left.chars().next().unwrap()),
                                comparison: match left[1..].chars().next().unwrap() {
                                    '>' => Ordering::Greater,
                                    '<' => Ordering::Less,
                                    _ => panic!(),
                                },
                                value: left[2..].parse().unwrap(),
                                to: to.to_string(),
                            }
                        } else {
                            Rule::Straight {
                                to: rule.to_string(),
                            }
                        }
                    })
                    .collect();
                (id.to_string(), Workflow(rules))
            })
            .collect();
        let parts = parts
            .lines()
            .map(|line| {
                let mut part = Part::default();
                for info in line[1..line.len() - 1].split(',') {
                    let (c, val) = info.split_once('=').unwrap();
                    let val = val.parse().unwrap();
                    part[CompType::from_char(c.chars().next().unwrap()) as usize] = val;
                }
                part
            })
            .collect();
        Ok((workflows, parts))
    }
    fn first((workflows, parts): Self::Parsed) -> Self::Output {
        parts
            .into_iter()
            .map(|part| {
                let mut bucket = "in".to_string();
                while !(bucket == "A" || bucket == "R") {
                    let workflow = &workflows[&bucket];
                    bucket = workflow.filter(&part);
                }
                if bucket == "A" {
                    part.iter().sum::<usize>()
                } else {
                    0
                }
            })
            .sum()
    }
    fn second((workflows, _): Self::Parsed) -> Self::Output {
        let mut part_ranges: SmallVec<[(String, PartRange); 16]> =
            smallvec![("in".to_string(), [(1, 4001); 4])];
        let mut combinations = 0;

        while let Some((bucket, range)) = part_ranges.pop() {
            part_ranges.extend(workflows[&bucket].ranges(range).into_iter().filter_map(
                |(bucket, range)| {
                    if bucket == "A" || bucket == "R" {
                        if bucket == "A" {
                            combinations += range.iter().map(|r| r.1 - r.0).product::<usize>();
                        }
                        None
                    } else {
                        Some((bucket, range))
                    }
                },
            ));
        }
        combinations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    fn parsed() -> <Day19 as Day>::Parsed {
        Day19::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day19::first(parsed()), 19114);
    }
    #[test]
    fn part2() {
        assert_eq!(Day19::second(parsed()), 167409079868000);
    }
}
