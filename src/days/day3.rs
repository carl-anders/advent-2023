use std::collections::HashMap;

use crate::helpers::grid2d::{parse_str_grid, Direction8Way, Position2D};

use super::day::Day;
use anyhow::Result;
use ndarray::Array2;

type Dir = Direction8Way;
type Pos = Position2D<usize>;

const fn is_symbol(c: char) -> bool {
    !matches!(c, '0'..='9' | '.')
}

pub struct Day3;
impl Day for Day3 {
    type Parsed = Array2<char>;
    type Output = u32;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(parse_str_grid(&input, |c| c)?)
    }
    fn first(input: Self::Parsed) -> Self::Output {
        let mut result = 0;

        let [height, width] = *input.shape() else {
            panic!()
        };
        for y in 0..height {
            let mut number = None;
            let mut touched_symbol = false;
            for x in 0..width {
                let c = input[(y, x)];
                if let '0'..='9' = c {
                    number = Some(number.unwrap_or(0) * 10 + c.to_digit(10).unwrap());
                    if !touched_symbol {
                        let pos = Pos::new(x, y);
                        for direction in Dir::EVERY {
                            if input.get((pos + direction).yx()).map(|c| is_symbol(*c))
                                == Some(true)
                            {
                                touched_symbol = true;
                            }
                        }
                    }
                } else {
                    if number.is_some() && touched_symbol {
                        result += number.unwrap();
                    }
                    number = None;
                    touched_symbol = false;
                }
            }
            if number.is_some() && touched_symbol {
                result += number.unwrap();
            }
        }

        result
    }
    fn second(input: Self::Parsed) -> Self::Output {
        let mut star_touchers: HashMap<Pos, Vec<u32>> = HashMap::new();

        let [height, width] = *input.shape() else {
            panic!()
        };
        for y in 0..height {
            let mut number = None;
            let mut touched_star = None;
            for x in 0..width {
                let c = input[(y, x)];
                if let '0'..='9' = c {
                    number = Some(number.unwrap_or(0) * 10 + c.to_digit(10).unwrap());
                    if touched_star.is_none() {
                        let pos = Pos::new(x, y);
                        for direction in Dir::EVERY {
                            if input.get((pos + direction).yx()) == Some(&'*') {
                                touched_star = Some(pos + direction);
                            }
                        }
                    }
                } else {
                    if let (Some(num), Some(star)) = (number, touched_star) {
                        star_touchers.entry(star).or_default().push(num);
                    }
                    number = None;
                    touched_star = None;
                }
            }
            if let (Some(num), Some(star)) = (number, touched_star) {
                star_touchers.entry(star).or_default().push(num);
            }
        }

        star_touchers
            .values()
            .map(|touchers| {
                if touchers.len() == 2 {
                    touchers[0] * touchers[1]
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    fn parsed() -> <Day3 as Day>::Parsed {
        Day3::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day3::first(parsed()), 4361);
    }
    #[test]
    fn part2() {
        assert_eq!(Day3::second(parsed()), 467835);
    }
}
