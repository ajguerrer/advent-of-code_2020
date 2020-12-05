use anyhow::{anyhow, Result};
use std::{fs, str::FromStr};

struct Password {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lower, s) = s.split_at(s.find('-').ok_or_else(|| anyhow!("could not find lower"))?);
        let s = &s[1..];
        let (upper, s) = s.split_at(s.find(' ').ok_or_else(|| anyhow!("could not find upper"))?);
        let s = &s[1..];
        let (letter, s) = s.split_at(
            s.find(": ")
                .ok_or_else(|| anyhow!("could not find letter"))?,
        );
        let password = &s[2..];

        Ok(Password {
            lower: lower.parse()?,
            upper: upper.parse()?,
            letter: letter.parse()?,
            password: password.parse()?,
        })
    }
}

pub fn day2_part1() -> Result<String> {
    let passwords = parse_file()?;
    let valid = passwords
        .iter()
        .filter(|p| (p.lower..=p.upper).contains(&p.password.matches(p.letter).count()))
        .count();

    Ok(format!("{} passwords are valid", valid))
}

pub fn day2_part2() -> Result<String> {
    let passwords = parse_file()?;
    let valid = passwords
        .iter()
        .filter(|p| {
            (p.password.chars().nth(p.lower - 1) == Some(p.letter))
                ^ (p.password.chars().nth(p.upper - 1) == Some(p.letter))
        })
        .count();

    Ok(format!("{} passwords are valid", valid))
}

fn parse_file() -> Result<Vec<Password>> {
    String::from_utf8_lossy(&fs::read("data/day2.txt")?)
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect()
}
