use std::fs;

use anyhow::Result;

pub fn day3_part1() -> Result<String> {
    let path = parse_file()?;
    Ok(format!("{} trees", arboreal_encounters(&path, 3, 1)))
}

pub fn day3_part2() -> Result<String> {
    let path = parse_file()?;
    let p1 = arboreal_encounters(&path, 1, 1);
    let p2 = arboreal_encounters(&path, 3, 1);
    let p3 = arboreal_encounters(&path, 5, 1);
    let p4 = arboreal_encounters(&path, 7, 1);
    let p5 = arboreal_encounters(&path, 1, 2);

    Ok(format!(
        "{} * {} * {} * {} * {} = {} trees",
        p1,
        p2,
        p3,
        p4,
        p5,
        p1 * p2 * p3 * p4 * p5
    ))
}

fn arboreal_encounters(path: &[String], right: usize, down: usize) -> usize {
    let (_, sum) = path
        .iter()
        .skip(down)
        .step_by(down)
        .fold((right, 0), |(pos, sum), line| {
            (
                pos + right,
                sum + usize::from(line.chars().nth(pos % line.len()) == Some('#')),
            )
        });
    sum
}

fn parse_file() -> Result<Vec<String>> {
    String::from_utf8_lossy(&fs::read("data/day3.txt")?)
        .lines()
        .map(|line| Ok(line.to_string()))
        .collect()
}
