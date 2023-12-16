use anyhow::{Context, Result};
use std::{collections::HashMap, fs};

use itertools::Itertools;
use serde_json::Value;

pub fn load() -> Result<HashMap<usize, [Option<String>; 2]>> {
    let data = fs::read_to_string("results.json")?;
    let v: Value = serde_json::from_str(&data)?;
    let mut days = HashMap::new();
    for (day, results) in v.as_object().context("Invalid results file")? {
        let day = day.parse::<usize>()?;

        let results: Vec<String> = results
            .as_array()
            .context(format!("Invalid results file for day {day}"))?
            .iter()
            .filter_map(|result| match result {
                Value::Bool(bool) => Some(bool.to_string()),
                Value::Number(num) => Some(num.to_string()),
                Value::String(str) => Some(str.clone()),
                Value::Array(vec) => Some(
                    vec.iter()
                        .map(|v| {
                            if let Value::String(s) = v {
                                s.clone()
                            } else {
                                v.to_string()
                            }
                        })
                        .join("\n"),
                ),
                _ => None,
            })
            .collect();
        if results.len() == 2 {
            days.insert(day, [Some(results[0].clone()), Some(results[1].clone())]);
        } else if results.len() == 1 {
            days.insert(day, [Some(results[0].clone()), None]);
        }
    }

    Ok(days)
}
