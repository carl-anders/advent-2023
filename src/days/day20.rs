use std::collections::{HashMap, VecDeque};

use super::day::Day;
use anyhow::Result;
use num_integer::Integer;

#[derive(Debug, Clone)]
pub enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    mt: ModuleType,
    send_to: Vec<String>,
}
impl Module {
    fn input(
        &mut self,
        from: &String,
        signal: bool,
        signals: &mut VecDeque<(String, bool, String)>,
    ) {
        match self.mt {
            ModuleType::Broadcast => {
                for to in &self.send_to {
                    signals.push_back((to.clone(), signal, self.name.clone()));
                }
            }
            ModuleType::FlipFlop(ref mut mem) => {
                if !signal {
                    *mem = !*mem;
                    for to in &self.send_to {
                        signals.push_back((to.clone(), *mem, self.name.clone()));
                    }
                }
            }
            ModuleType::Conjunction(ref mut mem) => {
                *mem.get_mut(from).unwrap() = signal;
                //dbg!(&mem);
                let pulse = !mem.iter().all(|(_, b)| *b);
                for to in &self.send_to {
                    signals.push_back((to.clone(), pulse, self.name.clone()));
                }
            }
        }
    }
    fn has_input(&mut self, input: String) {
        if let ModuleType::Conjunction(ref mut mem) = self.mt {
            mem.insert(input, false);
        }
    }
}

pub struct Day20;
impl Day for Day20 {
    type Parsed = HashMap<String, Module>;
    type Output = usize;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let (mut name, send_to) = line.split_once(" -> ").unwrap();
                let send_to = send_to
                    .split(", ")
                    .map(std::string::ToString::to_string)
                    .collect();
                let mt = match name.chars().next().unwrap() {
                    '%' => {
                        name = &name[1..];
                        ModuleType::FlipFlop(false)
                    }
                    '&' => {
                        name = &name[1..];
                        ModuleType::Conjunction(HashMap::new())
                    }
                    'b' => ModuleType::Broadcast,
                    _ => panic!(),
                };
                (
                    name.to_string(),
                    Module {
                        name: name.to_string(),
                        mt,
                        send_to,
                    },
                )
            })
            .collect())
    }
    fn first(mut modules: Self::Parsed) -> Self::Output {
        for (name, module) in modules.clone() {
            for to in module.send_to {
                if let Some(m) = modules.get_mut(&to) {
                    m.has_input(name.clone());
                }
            }
        }
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        for _ in 0..1000 {
            let mut signals = VecDeque::new();
            signals.push_back(("broadcaster".to_string(), false, "button".to_string()));
            while let Some((to, signal, from)) = signals.pop_front() {
                if let Some(module) = modules.get_mut(&to) {
                    module.input(&from, signal, &mut signals);
                }
                if signal {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
            }
        }
        low_pulses * high_pulses
    }
    fn second(mut modules: Self::Parsed) -> Self::Output {
        for (name, module) in modules.clone() {
            for to in module.send_to {
                if let Some(m) = modules.get_mut(&to) {
                    m.has_input(name.clone());
                }
            }
        }

        let mut end_modules: Vec<HashMap<String, usize>> = modules.iter().filter_map(|(_, module)| {
            if module.send_to.contains(&"rx".to_string()) {
                match &module.mt {
                    ModuleType::Conjunction(mem) => Some(mem.keys().map(|s| (s.to_string(), 0)).collect()),
                    _ => {
                        panic!("Day 20 Part 2 checker only supports Conjunction module for end mt sender.")
                    }
                }
            } else {
                None
            }
        }).collect();
        assert!(end_modules.len() == 1, "Day 20 Part 2 checker only supports one end module.");
        let mut cycles = end_modules.pop().unwrap();

        let mut presses = 0;
        loop {
            presses += 1;
            let mut signals = VecDeque::new();
            signals.push_back(("broadcaster".to_string(), false, "button".to_string()));
            while let Some((to, signal, from)) = signals.pop_front() {
                if let Some(module) = modules.get_mut(&to) {
                    module.input(&from, signal, &mut signals);
                }
                if !signal {
                    if let Some(cycle) = cycles.get_mut(&to) {
                        *cycle = presses;
                    }
                }
                if !signal && to == "rx" {
                    return presses;
                }
            }
            if cycles.values().all(|&cycle| cycle > 0) {
                return cycles.values().fold(1, |acc, cycle| acc.lcm(cycle));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    fn parsed() -> <Day20 as Day>::Parsed {
        Day20::parse(INPUT.to_string()).unwrap()
    }
    fn parsed2() -> <Day20 as Day>::Parsed {
        Day20::parse(INPUT2.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day20::first(parsed()), 32000000);
        assert_eq!(Day20::first(parsed2()), 11687500);
    }
}
