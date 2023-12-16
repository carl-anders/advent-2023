use super::day::Day;
use anyhow::Result;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Map {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}
#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<i64>,
    maps_list: Vec<Vec<Map>>,
}

pub struct Day5;
impl Day for Day5 {
    type Parsed = Almanac;
    type Output = i64;

    fn parse(input: String) -> Result<Self::Parsed> {
        let (seeds, maps) = input.split_once("\n\n").unwrap();
        let seeds = seeds.split(' ').filter_map(|s| s.parse().ok()).collect();
        let maps = maps
            .split("\n\n")
            .map(|map| {
                map.lines()
                    .skip(1)
                    .map(|line| {
                        let mut nums = line.split(' ').map(|num| num.parse::<i64>().unwrap());
                        Map {
                            destination_start: nums.next().unwrap(),
                            source_start: nums.next().unwrap(),
                            range_length: nums.next().unwrap(),
                        }
                    })
                    .collect()
            })
            .collect();
        Ok(Almanac { seeds, maps_list: maps })
    }
    fn first(almanac: Self::Parsed) -> Self::Output {
        let mut lowest = i64::MAX;
        for mut seed in almanac.seeds {
            for maps in &almanac.maps_list {
                for map in maps {
                    let range = map.source_start..(map.source_start+map.range_length);
                    if range.contains(&seed) {
                        seed += map.destination_start - map.source_start;
                        break;
                    }
                }
            }
            if seed < lowest {
                lowest = seed;
            }
        }
        lowest
    }
    fn second(almanac: Self::Parsed) -> Self::Output {
        let ranges: Vec<[i64;2]> = almanac.seeds.chunks_exact(2).map(|a| [a[0], a[1]]).collect();
        ranges.into_par_iter().map(|seed_range| {
            println!("Testing seed range: {seed_range:?}");
            let mut lowest = i64::MAX;
            for mut seed in seed_range[0]..(seed_range[0]+seed_range[1]) {

                for maps in &almanac.maps_list {
                    for map in maps {
                        let range = map.source_start..(map.source_start+map.range_length);
                        if range.contains(&seed) {
                            seed += map.destination_start - map.source_start;
                            break;
                        }
                    }
                }
                if seed < lowest {
                    lowest = seed;
                }
            }
            println!("Lowest: {lowest:?}");
            lowest
        }).min().unwrap()
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
