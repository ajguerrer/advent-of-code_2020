use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let mut adapters = parse_file();
    adapters.sort_unstable();
    count_adapters(&adapters, 1) * count_adapters(&adapters, 3)
}

fn count_adapters(adapters: &[u32], joltage: u32) -> usize {
    adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .filter(|(a1, a2)| *a2 - *a1 == joltage)
        .count()
        + 1
}

fn part2() -> usize {
    let mut adapters = parse_file();
    adapters.push(0);
    adapters.sort_unstable();
    let mut running_total = vec![1; adapters.len()];
    for i in 1..adapters.len() {
        running_total[i] = (i.saturating_sub(3)..i)
            .filter(|j| adapters[i] - adapters[*j] <= 3)
            .map(|j| running_total[j])
            .sum();
    }
    *running_total.last().unwrap()
}

fn parse_file() -> Vec<u32> {
    read_to_string("data/day10.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
