use anyhow::{bail, Result};
use std::fs;

pub fn day5_part1() -> Result<String> {
    let passes = parse_file()?;
    let max = passes.iter().max().unwrap();
    Ok(format!("highest pass is {}", max))
}

pub fn day5_part2() -> Result<String> {
    let passes = parse_file()?;
    let front_seats = (1..(2usize.pow(8) * 8 - 1))
        .find(|p| passes.contains(p))
        .unwrap_or_default();
    let next_open_seat = (front_seats..(2usize.pow(8) * 8 - 1))
        .find(|p| !passes.contains(p))
        .unwrap_or_default();

    Ok(format!("next open seat from front {:?}", next_open_seat))
}

fn decode_pass(s: &str) -> Result<usize> {
    if s.chars().count() != 10 {
        bail!("pass must be 10 chars");
    }
    let (row, seat) = s.split_at(7);
    let row = row
        .chars()
        .rev()
        .enumerate()
        .try_fold(0, |row, (i, c)| match c {
            'B' => Ok(row + 2usize.pow(i as u32)),
            'F' => Ok(row),
            _ => bail!("invalid row char"),
        })?;
    let seat = seat
        .chars()
        .rev()
        .enumerate()
        .try_fold(0, |seat, (i, c)| match c {
            'R' => Ok(seat + 2usize.pow(i as u32)),
            'L' => Ok(seat),
            _ => bail!("invalid seat char"),
        })?;
    Ok(row * 8 + seat)
}

fn parse_file() -> Result<Vec<usize>> {
    String::from_utf8_lossy(&fs::read("data/day5.txt")?)
        .lines()
        .map(|line| Ok(decode_pass(line)?))
        .collect()
}
