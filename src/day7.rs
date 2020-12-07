use anyhow::{anyhow, Result};
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Bag {
    name: String,
    contains: Vec<(u32, String)>,
}

impl FromStr for Bag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, contains) = s.split_at(
            s.find(" bags contain ")
                .ok_or_else(|| anyhow!("could not find contents"))?,
        );
        let name = name.to_string();
        let contains = contains.trim_start_matches(" bags contain ");

        if contains.contains(" no other bags") {
            return Ok(Bag {
                name,
                contains: vec![],
            });
        }

        let contains = contains
            .split(", ")
            // .map(|s| s.to_string())
            .filter_map(|s| {
                let (count, name) = s.split_at(s.find(' ')?);
                let count: u32 = count.parse().ok()?;
                let name = name
                    .trim()
                    .split_whitespace()
                    .take(2)
                    .collect::<Vec<_>>()
                    .join(" ");
                Some((count, name))
            })
            .collect();
        Ok(Bag { name, contains })
    }
}

pub fn day7_part1() -> Result<String> {
    let bags = parse_file()?;
    let mut names = vec!["shiny gold".to_string()];
    let mut prev = 0;
    while prev != names.len() {
        prev = names.len();
        let new: Vec<_> = bags
            .iter()
            .filter_map(|b| {
                if b.contains
                    .iter()
                    .any(|(_, name)| names.contains(name) && !names.contains(&b.name))
                {
                    Some(b.name.clone())
                } else {
                    None
                }
            })
            .collect();
        names.extend(new);
    }
    Ok(format!("{}", names.len() - 1))
}

pub fn day7_part2() -> Result<String> {
    let bags = parse_file()?;
    let mut current = vec![(1, bags.iter().find(|b| b.name == "shiny gold").unwrap())];
    let mut sum = 0;
    while !current.is_empty() {
        sum += current.iter().fold(0, |sum, (num, b)| {
            sum + num
                * b.contains
                    .iter()
                    .fold(0, |icount, (bcount, _)| icount + bcount)
        });
        dbg!(sum);
        current = current.iter().fold(vec![], |mut current, (count, cb)| {
            current.extend(
                cb.contains
                    .iter()
                    .map(|(num, name)| {
                        (*num * count, bags.iter().find(|b| &b.name == name).unwrap())
                    })
                    .collect::<Vec<_>>(),
            );
            current
        });
        dbg!(&current);
    }
    Ok(format!("{}", sum,))
}

fn parse_file() -> Result<Vec<Bag>> {
    String::from_utf8_lossy(&fs::read("data/day7.txt")?)
        .lines()
        .map(|rule| Ok(rule.parse()?))
        .collect()
}
