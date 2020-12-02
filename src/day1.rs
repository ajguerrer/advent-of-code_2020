use std::fs;

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub fn day1_part1() -> Result<String> {
    let nums = parse_file()?;
    let (n1, n2) = nums
        .iter()
        .enumerate()
        .find_map(|(i, n1)| Some((n1, nums[i..].iter().find(|n2| n1 + *n2 == 2020)?)))
        .ok_or_else(|| anyhow!("solution not found"))?;

    Ok(format!("{} * {} = {}", n1, n2, n1 * n2))
}

pub fn day1_part2() -> Result<String> {
    let nums = parse_file()?;
    let (n1, n2, n3) = nums
        .iter()
        .enumerate()
        .find_map(|(i, n1)| {
            let (n2, n3) = nums[i..].iter().enumerate().find_map(|(j, n2)| {
                Some((n2, nums[j..].iter().find(|n3| n1 + n2 + *n3 == 2020)?))
            })?;
            Some((n1, n2, n3))
        })
        .ok_or_else(|| anyhow!("solution not found"))?;

    Ok(format!("{} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3))
}

pub fn day1_part1_easy_mode() -> Result<String> {
    let nums = parse_file()?;
    let (n1, n2) = nums
        .iter()
        .tuple_combinations()
        .find(|(n1, n2)| *n1 + *n2 == 2020)
        .ok_or_else(|| anyhow!("solution not found"))?;

    Ok(format!("{} * {} = {}", n1, n2, n1 * n2))
}

pub fn day1_part2_easy_mode() -> Result<String> {
    let nums = parse_file()?;
    let (n1, n2, n3) = nums
        .iter()
        .tuple_combinations()
        .find(|(n1, n2, n3)| *n1 + *n2 + *n3 == 2020)
        .ok_or_else(|| anyhow!("solution not found"))?;

    Ok(format!("{} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3))
}

fn parse_file() -> Result<Vec<u32>> {
    String::from_utf8_lossy(&fs::read("data/day1.txt")?)
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect()
}
