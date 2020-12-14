#![feature(str_split_once)]

use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> u32 {
    let (arrival, sched) = parse_file();
    let (bus, wait) = sched
        .iter()
        .filter_map(|bus| *bus)
        .fold(None, |min, bus| {
            let wait = ((arrival / bus) + 1) * bus - arrival;
            match min {
                Some((_, w)) if wait < w => Some((bus, wait)),
                None => Some((bus, wait)),
                min => min,
            }
        })
        .unwrap();
    bus * wait
}

fn part2() -> usize {
    let (_, sched) = parse_file();
    let (solution, _) = sched
        .iter()
        .enumerate()
        .filter_map(|(stagger_by, bus)| bus.map(|bus| (stagger_by, bus as usize)))
        .fold((0, 1), |(acc, running_period), (stagger_by, bus)| {
            let mut acc = acc;
            while (acc + stagger_by) % bus != 0 {
                acc += running_period;
            }
            (acc, running_period * bus)
        });
    solution
}

fn parse_file() -> (u32, Vec<Option<u32>>) {
    let input = read_to_string("data/day13.txt").unwrap();
    let (arrival, sched) = input.split_once('\n').unwrap();
    let sched = sched.split(',').map(|bus| bus.parse().ok()).collect();
    (arrival.parse().unwrap(), sched)
}
