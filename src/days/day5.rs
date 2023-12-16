use std::ops::Range;

use crate::helpers::RangeIntersect;

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Clone)]
pub struct Map {
    range: Range<i64>,
    destination: i64,
}
#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<i64>,
    maps_list: Vec<Vec<Map>>,
}

#[derive(Debug, Clone)]
pub struct Ranger {
    ranges: SmallVec<[Range<i64>; 32]>,
}
impl Ranger {
    fn new(range: Range<i64>) -> Self {
        Self {
            ranges: smallvec![range],
        }
    }
    fn apply_maps(&mut self, maps: &Vec<Map>) {
        let mut finished_ranges: SmallVec<[_; 32]> = SmallVec::new();
        for map in maps {
            let mut outside_ranges: SmallVec<[_; 32]> = SmallVec::new();
            while let Some(range) = self.ranges.pop() {
                // Is the range to the "left" of the map
                if range.start < map.range.start {
                    // Add part of the range that is to the "left" of the map
                    outside_ranges.push(range.start..range.end.min(map.range.start));
                }
                // Does the range intersect with the map
                if let Some(intersect) = map.range.intersect(&range) {
                    // The interecting range is changed according to the map
                    let diff = map.destination - map.range.start;
                    finished_ranges.push(intersect.start + diff..intersect.end + diff);
                }
                // Is the range to the "right" of the map
                if range.end > map.range.end {
                    // Add part of the range that is to the "right" of the map
                    outside_ranges.push(map.range.end.max(range.start)..range.end);
                }
            }
            self.ranges = outside_ranges;
        }
        self.ranges.append(&mut finished_ranges);
    }
    fn min(&self) -> i64 {
        self.ranges.iter().map(|r| r.start).min().unwrap()
    }
}

pub struct Day5;
impl Day for Day5 {
    type Parsed = Almanac;
    type Output = i64;

    fn parse(input: String) -> Result<Self::Parsed> {
        let (seeds, maps) = input.split_once("\n\n").unwrap();
        let seeds = seeds.split(' ').filter_map(|s| s.parse().ok()).collect();
        let maps_list = maps
            .split("\n\n")
            .map(|map| {
                map.lines()
                    .skip(1)
                    .filter_map(|line| {
                        let nums: (i64, i64, i64) = line
                            .split(' ')
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect_tuple()?;
                        Some(Map {
                            range: nums.1..(nums.1 + nums.2),
                            destination: nums.0,
                        })
                    })
                    .collect()
            })
            .collect();
        Ok(Almanac { seeds, maps_list })
    }
    fn first(almanac: Self::Parsed) -> Self::Output {
        almanac
            .seeds
            .into_iter()
            .map(|mut seed| {
                for maps in &almanac.maps_list {
                    for map in maps {
                        if map.range.contains(&seed) {
                            seed += map.destination - map.range.start;
                            break;
                        }
                    }
                }
                seed
            })
            .min()
            .unwrap()
    }
    fn second(almanac: Self::Parsed) -> Self::Output {
        almanac
            .seeds
            .chunks_exact(2)
            .map(|chunk| {
                let mut ranger = Ranger::new(chunk[0]..chunk[0] + chunk[1]);
                for maps in &almanac.maps_list {
                    ranger.apply_maps(maps);
                }
                ranger.min()
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    fn parsed() -> <Day5 as Day>::Parsed {
        Day5::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day5::first(parsed()), 35);
    }
    #[test]
    fn part2() {
        assert_eq!(Day5::second(parsed()), 46);
    }
}
