use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let groups = parse_file();
    groups
        .iter()
        .map(|g| ('a'..='z').filter(|c| g.contains(*c)).count())
        .sum()
}

fn part2() -> usize {
    let groups = parse_file();
    groups
        .iter()
        .map(|g| {
            ('a'..='z')
                .filter(|c| g.lines().all(|p| p.contains(*c)))
                .count()
        })
        .sum()
}

fn parse_file() -> Vec<String> {
    read_to_string("data/day06.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| group.to_string())
        .collect()
}
