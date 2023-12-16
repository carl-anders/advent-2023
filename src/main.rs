#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::option_if_let_else,
    clippy::wildcard_imports,
    clippy::manual_range_contains,
    clippy::similar_names,
    clippy::enum_glob_use,
    clippy::cast_sign_loss
)]

use itertools::Itertools;
use std::{
    collections::HashMap,
    fs,
    time::{Duration, Instant},
};
mod days;
use days::*;
mod helpers;
mod results;

const TEXT_RED: &str = "\x1b[1;31m";
const TEXT_YELLOW: &str = "\x1b[1;33m";
const TEXT_RESET: &str = "\x1b[0m";
const TEXT_HEADER: &str = "\x1b[2;30;47m";

fn header(header: &str) {
    println!("\n{TEXT_HEADER}{header:#^60}{TEXT_RESET}");
}

fn run<T: day::Day>(
    file: &str,
    results: &[Option<String>; 2],
) -> Option<(
    std::time::Duration,
    std::time::Duration,
    std::time::Duration,
)> {
    let Ok(input) = fs::read_to_string(file) else {
        println!("{TEXT_YELLOW}No input file found:{TEXT_RESET}");
        return None;
    };

    let start_time = Instant::now();
    let parsed = T::parse(input);
    let parsed_time = start_time.elapsed();
    let parsed = parsed.expect(&format!("Error parsing file {file}")[..]);

    let mut times = (0..=1).map(|i| {
        println!("- Part {}:", i + 1);
        let cloned = parsed.clone();
        let start_time = Instant::now();
        let result = if i == 0 {
            T::first(cloned)
        } else {
            T::second(cloned)
        };
        let elapsed = start_time.elapsed();
        println!("-- Result:\n{}", result.to_string());
        match &results[i] {
            Some(saved_result) => {
                if result.to_string() != *saved_result {
                    println!("{TEXT_RED}Results do not match! Stored result:{TEXT_RESET}\n{saved_result}");
                }
            }
            None => {
                println!("{TEXT_YELLOW}New result found!{TEXT_RESET}");
            }
        }
        elapsed
    });

    Some((parsed_time, times.next().unwrap(), times.next().unwrap()))
}

fn run_day(day: u8, results: &[Option<String>; 2]) -> Option<(Duration, Duration, Duration)> {
    let file = format!("input/{day}.txt");
    header(&format!(" Day {day} "));
    match day {
        1 => run::<day1::Day1>(&file, results),
        2 => run::<day2::Day2>(&file, results),
        3 => run::<day3::Day3>(&file, results),
        4 => run::<day4::Day4>(&file, results),
        5 => run::<day5::Day5>(&file, results),
        6 => run::<day6::Day6>(&file, results),
        7 => run::<day7::Day7>(&file, results),
        8 => run::<day8::Day8>(&file, results),
        9 => run::<day9::Day9>(&file, results),
        10 => run::<day10::Day10>(&file, results),
        11 => run::<day11::Day11>(&file, results),
        12 => run::<day12::Day12>(&file, results),
        13 => run::<day13::Day13>(&file, results),
        14 => run::<day14::Day14>(&file, results),
        15 => run::<day15::Day15>(&file, results),
        16 => run::<day16::Day16>(&file, results),
        17 => run::<day17::Day17>(&file, results),
        18 => run::<day18::Day18>(&file, results),
        19 => run::<day19::Day19>(&file, results),
        20 => run::<day20::Day20>(&file, results),
        21 => run::<day21::Day21>(&file, results),
        22 => run::<day22::Day22>(&file, results),
        23 => run::<day23::Day23>(&file, results),
        24 => run::<day24::Day24>(&file, results),
        25 => run::<day25::Day25>(&file, results),
        _ => panic!(),
    }
}

fn run_days(days: Vec<u8>) {
    let mut timings: Vec<(u8, (Duration, Duration, Duration))> = Vec::new();

    let results = results::load().unwrap_or_else(|e| {
        println!("{TEXT_YELLOW}Can't load results.json file:{TEXT_RESET}\n  {e:?}");
        HashMap::new()
    });
    for day in days {
        let result = results.get(&(day as usize)).unwrap_or(&[None, None]);
        if let Some(t) = run_day(day, result) {
            timings.push((day, t));
        }
    }
    timings = timings
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&(b.1 .0 + b.1 .1 + b.1 .2), &(a.1 .0 + a.1 .1 + a.1 .2)))
        .collect();

    header(" TIMINGS ");
    for chunk in timings.chunks(7) {
        print!("Day:     ");
        for (name, _) in chunk {
            print!("|{name:^15}");
        }
        print!("|\nParsing: ");
        for (_, (a, _, _)) in chunk {
            print!("|{:^15}", format!("{a:?}"));
        }
        print!("|\nPart 1:  ");
        for (_, (_, b, _)) in chunk {
            print!("|{:^15}", format!("{b:?}"));
        }
        print!("|\nPart 2:  ");
        for (_, (_, _, c)) in chunk {
            print!("|{:^15}", format!("{c:?}"));
        }
        println!("|");
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            run_days((1..=25).collect());
        }
        2 => match args[1].parse() {
            Ok(day) if (1..=25).contains(&day) => {
                run_days(vec![day]);
            }
            _ => panic!("Wrong day passed"),
        },
        _ => panic!("Wrong parameters"),
    }
}
