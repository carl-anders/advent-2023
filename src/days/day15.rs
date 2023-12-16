use std::string::ToString;

use super::day::Day;
use anyhow::Result;
use smallvec::SmallVec;
use smallvec::smallvec;

fn hash_code(input: &str) -> usize {
    let mut val = 0;
    for &c in input.as_bytes() {
        val += c as usize;
        val *= 17;
        val %= 256;
    }
    val
}

pub struct Day15;
impl Day for Day15 {
    type Parsed = Vec<String>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .trim_end()
            .split(',')
            .map(ToString::to_string)
            .collect())
    }
    fn first(codes: Self::Parsed) -> Self::Output {
        codes.iter().map(|code| hash_code(code)).sum()
    }
    fn second(codes: Self::Parsed) -> Self::Output {
        let mut boxes: SmallVec<[SmallVec<[(String, usize); 16]>; 256]> = SmallVec::from_elem(smallvec![], 256);

        for code in codes {
            let (label, focal_length) = code.split_once(&['-', '='][..]).unwrap();
            let hash = hash_code(label);
            let boxx = boxes.get_mut(hash).unwrap();
            if code.contains('=') {
                let focal_length = focal_length.parse().unwrap();
                if let Some(slot) = boxx.iter_mut().find(|(find_label, _)| find_label == label) {
                    if slot.0 == label {
                        slot.1 = focal_length;
                    }
                } else {
                    boxx.push((label.to_string(), focal_length));
                }
            } else {
                boxx.retain(|(find_label, _)| label != find_label);
            }
        }

        boxes
            .iter()
            .map(|boxx| {
                boxx.iter()
                    .enumerate()
                    .map(|(slot, (label, focal_length))| {
                        (hash_code(label) + 1) * (slot + 1) * focal_length
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    fn parsed() -> <Day15 as Day>::Parsed {
        Day15::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day15::first(parsed()), 1320);
    }
    #[test]
    fn part2() {
        assert_eq!(Day15::second(parsed()), 145);
    }
}
