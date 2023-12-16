use std::collections::HashMap;

use super::day::Day;
use anyhow::Result;
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use smallvec::smallvec;
use smallvec::SmallVec;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Location {
    Operational,
    Damaged,
    Unknown,
}
impl Location {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct SpringRow {
    springs: SmallVec<[Location; 128]>,
    counts: SmallVec<[usize; 32]>,
}

impl SpringRow {
    fn poss(
        &self,
        cache: &mut HashMap<(usize, usize, usize), usize>,
        position: usize,
        block_pos: usize,
        block_len: usize,
    ) -> usize {
        use Location::*;
        let key = (position, block_pos, block_len);
        if let Some(val) = cache.get(&key) {
            return *val;
        }

        if position == self.springs.len() {
            if (block_pos == self.counts.len() && block_len == 0)
                || (block_pos == self.counts.len() - 1 && self.counts[block_pos] == block_len)
            {
                return 1;
            }
            return 0;
        }

        let mut sum = 0;
        if self.springs[position] == Damaged || self.springs[position] == Unknown {
            sum += self.poss(cache, position + 1, block_pos, block_len + 1);
        }
        if self.springs[position] == Operational || self.springs[position] == Unknown {
            if block_len == 0 {
                sum += self.poss(cache, position + 1, block_pos, 0);
            } else if block_len > 0
                && block_pos < self.counts.len()
                && self.counts[block_pos] == block_len
            {
                sum += self.poss(cache, position + 1, block_pos + 1, 0);
            }
        }

        cache.insert(key, sum);
        sum
    }
    fn possibilities(&self) -> usize {
        self.poss(&mut HashMap::new(), 0, 0, 0)
    }
    fn expand(&mut self) {
        self.springs = (0..9).map(|n| {
            if n % 2 == 0 {
                self.springs.clone()
            } else {
                smallvec![Location::Unknown]
            }
        }).concat();

        self.counts = self.counts.repeat(5).into();
    }
}

pub struct Day12;
impl Day for Day12 {
    type Parsed = Vec<SpringRow>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let (springs, counts) = line.split_once(' ').unwrap();
                let springs = springs.chars().map(Location::from_char).collect();
                let counts = counts
                    .split(',')
                    .filter_map(|count| count.parse().ok())
                    .collect();
                SpringRow { springs, counts }
            })
            .collect())
    }
    fn first(rows: Self::Parsed) -> Self::Output {
        rows.par_iter().map(SpringRow::possibilities).sum()
    }
    fn second(mut rows: Self::Parsed) -> Self::Output {
        rows.par_iter_mut()
            .map(|row| {
                row.expand();
                row.possibilities()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    fn parsed() -> <Day12 as Day>::Parsed {
        Day12::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day12::first(parsed()), 21);
    }
    #[test]
    fn part2() {
        assert_eq!(Day12::second(parsed()), 525152);
    }
}
