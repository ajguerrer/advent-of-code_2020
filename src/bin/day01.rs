use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> u32 {
    let nums = parse_file();
    let (n1, n2) = nums
        .iter()
        .enumerate()
        .find_map(|(i, n1)| Some((n1, nums[i..].iter().find(|n2| n1 + *n2 == 2020)?)))
        .unwrap();
    n1 * n2
}

fn part2() -> u32 {
    let nums = parse_file();
    let (n1, n2, n3) = nums
        .iter()
        .enumerate()
        .find_map(|(i, n1)| {
            let (n2, n3) = nums[i..].iter().enumerate().find_map(|(j, n2)| {
                Some((n2, nums[j..].iter().find(|n3| n1 + n2 + *n3 == 2020)?))
            })?;
            Some((n1, n2, n3))
        })
        .unwrap();
    n1 * n2 * n3
}

fn parse_file() -> Vec<u32> {
    read_to_string("data/day01.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
