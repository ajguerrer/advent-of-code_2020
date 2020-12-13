#![feature(str_split_once)]

use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let (arrival, sched) = parse_file();
    0
}

fn part2() -> usize {
    let _ = parse_file();
    0
}

fn parse_file() -> (u32, Vec<u32>) {
    let input = read_to_string("data/day13.txt").unwrap();
    let (arrival, sched) = input.split_once('\n').unwrap();
    let sched = sched
        .split(',')
        .filter(|bus| bus != &"x")
        .map(|bus| bus.parse().unwrap())
        .collect();
    (arrival.parse().unwrap(), sched)
}
