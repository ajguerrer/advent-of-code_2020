#![feature(str_split_once)]

use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> u64 {
    let v = parse_file();
    v.iter().map(|s| evaluate(&reduce_paren(s, false))).sum()
}

fn part2() -> u64 {
    let v = parse_file();
    v.iter()
        .map(|s| evaluate(&(reduce_add(&reduce_paren(s, true)))))
        .sum()
}

fn reduce_paren(s: &str, add_first: bool) -> String {
    let mut s = s.to_string();
    while let Some((expr, paren)) = s.rsplit_once('(') {
        let (paren, rest) = paren.split_once(')').unwrap();
        s = if add_first {
            expr.to_string() + &evaluate(&reduce_add(paren)).to_string() + rest
        } else {
            expr.to_string() + &evaluate(paren).to_string() + rest
        };
    }
    s
}

fn evaluate(s: &str) -> u64 {
    s.split(' ')
        .fold((0, "+"), |(lhs, op), s| match s {
            "+" => (lhs, "+"),
            "*" => (lhs, "*"),
            num => match op {
                "+" => (lhs + num.parse::<u64>().unwrap(), op),
                "*" => (lhs * num.parse::<u64>().unwrap(), op),
                op => panic!("weird op {}", op),
            },
        })
        .0
}

fn reduce_add(s: &str) -> String {
    let mut s = s.to_string();
    while let Some((lhs, rhs)) = s.split_once(" + ") {
        let (prev, lhs) = lhs.rsplit_once(' ').unwrap_or(("", lhs));
        let (rhs, rest) = rhs.split_once(' ').unwrap_or((rhs, ""));
        let product = evaluate(&(lhs.to_string() + " + " + rhs)).to_string();
        s = [prev, " ", &product, " ", rest].concat().trim().to_string();
    }
    s
}

fn parse_file() -> Vec<String> {
    read_to_string("data/day18.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}
