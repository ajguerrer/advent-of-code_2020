use anyhow::Result;
use std::fs;

pub fn day6_part1() -> Result<String> {
    let groups = parse_file()?;
    let total: usize = groups
        .iter()
        .map(|g| ('a'..='z').filter(|c| g.contains(*c)).count())
        .sum();
    Ok(format!("sum of any {:?}", total))
}

pub fn day6_part2() -> Result<String> {
    let groups = parse_file()?;
    let total: usize = groups
        .iter()
        .map(|g| {
            ('a'..='z')
                .filter(|c| g.lines().all(|p| p.contains(*c)))
                .count()
        })
        .sum();
    Ok(format!("sum of all {:?}", total))
}

fn parse_file() -> Result<Vec<String>> {
    String::from_utf8_lossy(&fs::read("data/day6.txt")?)
        .split("\n\n")
        .map(|group| Ok(group.to_string()))
        .collect()
}
