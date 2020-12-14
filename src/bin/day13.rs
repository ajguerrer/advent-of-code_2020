#![feature(str_split_once)]

use std::fs::read_to_string;

fn main() {
    // println!("{}", part1());
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
    let max = sched.iter().max().unwrap().unwrap();
    let i = sched.iter().position(|b| b == &Some(max)).unwrap();
    (max as usize - i..)
        .step_by(max as usize)
        .find(|mul| is_wave(*mul, &sched))
        .unwrap()
}

fn is_wave(mul: usize, sched: &[Option<u32>]) -> bool {
    dbg!(mul);
    sched.iter().enumerate().all(|(i, b)| match b {
        Some(b) => (mul + i) % *b as usize == 0,
        None => true,
    })
}

fn parse_file() -> (u32, Vec<Option<u32>>) {
    let input = read_to_string("data/day13.txt").unwrap();
    let (arrival, sched) = input.split_once('\n').unwrap();
    let sched = sched.split(',').map(|bus| bus.parse().ok()).collect();
    (arrival.parse().unwrap(), sched)
}
