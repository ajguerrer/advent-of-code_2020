#![feature(str_split_once)]

use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let passwords = parse_file();
    passwords
        .iter()
        .filter(|p| (p.lower..=p.upper).contains(&p.password.matches(p.letter).count()))
        .count()
}

fn part2() -> usize {
    let passwords = parse_file();
    passwords
        .iter()
        .filter(|p| {
            (p.password.chars().nth(p.lower - 1).unwrap() == p.letter)
                ^ (p.password.chars().nth(p.upper - 1).unwrap() == p.letter)
        })
        .count()
}

struct Password {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

impl Password {
    fn new(s: &str) -> Self {
        let (lower, s) = s.split_once('-').unwrap();
        let (upper, s) = s.split_once(' ').unwrap();
        let (letter, password) = s.split_once(": ").unwrap();

        Password {
            lower: lower.parse().unwrap(),
            upper: upper.parse().unwrap(),
            letter: letter.parse().unwrap(),
            password: password.parse().unwrap(),
        }
    }
}

fn parse_file() -> Vec<Password> {
    read_to_string("data/day02.txt")
        .unwrap()
        .lines()
        .map(|line| Password::new(line))
        .collect()
}
