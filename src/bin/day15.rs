use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    memory_game(&parse_file(), 2020)
}

fn part2() -> usize {
    memory_game(&parse_file(), 30000000)
}

fn memory_game(nums: &[usize], rounds: usize) -> usize {
    let map =
        nums.iter()
            .enumerate()
            .take(nums.len() - 1)
            .fold(HashMap::new(), |mut map, (i, n)| {
                map.insert(*n, i + 1);
                map
            });
    let last = *nums.last().unwrap();
    let (last, _) = (nums.len()..rounds).fold((last, map), |(last, mut map), i| {
        let spoken = match map.get(&last) {
            None => 0usize,
            Some(turn) => i - turn,
        };
        map.insert(last, i);
        (spoken, map)
    });
    last
}

fn parse_file() -> Vec<usize> {
    read_to_string("data/day15.txt")
        .unwrap()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect()
}
