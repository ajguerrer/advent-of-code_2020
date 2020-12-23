#![feature(str_split_once)]

use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let (rules, messages) = parse_file();
    count_matches(&rules, &messages)
}

fn part2() -> usize {
    let (mut rules, messages) = parse_file();
    rules.insert(8, Rule::Either((vec![42], vec![42, 8])));
    rules.insert(11, Rule::Either((vec![42, 31], vec![42, 11, 31])));
    count_matches(&rules, &messages)
}

fn count_matches(rules: &Rules, messages: &[String]) -> usize {
    messages
        .iter()
        .flat_map(|message| strip_rule(message, 0, &rules))
        .filter(|message| message.is_empty())
        .count()
}

fn strip_rule<'a>(message: &'a str, rule: u32, rules: &Rules) -> Vec<&'a str> {
    if message.is_empty() {
        return Vec::new();
    };

    match rules.get(&rule).unwrap() {
        Rule::Value(c) => message.strip_prefix(*c).map_or(Vec::new(), |m| vec![m]),
        Rule::Sequence(s) => strip_sequence(message, s, rules),
        Rule::Either((l, r)) => [
            strip_sequence(message, l, rules),
            strip_sequence(message, r, rules),
        ]
        .concat(),
    }
}

fn strip_sequence<'a>(message: &'a str, seq: &[u32], rules: &Rules) -> Vec<&'a str> {
    seq.iter().fold(vec![message], |messages, rule| {
        messages
            .iter()
            .flat_map(|message| strip_rule(message, *rule, rules))
            .collect()
    })
}

type Rules = HashMap<u32, Rule>;

#[derive(Debug)]
enum Rule {
    Value(char),
    Sequence(Vec<u32>),
    Either((Vec<u32>, Vec<u32>)),
}

fn parse_rules(s: &str) -> Rules {
    s.lines().fold(HashMap::new(), |mut map, s| {
        let (num, rule) = s.split_once(": ").unwrap();
        let rule = if rule.starts_with('"') {
            Rule::Value(rule.chars().nth(1).unwrap())
        } else if rule.contains('|') {
            let (lhs, rhs) = rule.split_once(" | ").unwrap();
            Rule::Either((
                (lhs.split(' ').map(|n| n.parse().unwrap()).collect()),
                (rhs.split(' ').map(|n| n.parse().unwrap()).collect()),
            ))
        } else {
            Rule::Sequence(rule.split(' ').map(|n| n.parse().unwrap()).collect())
        };
        map.insert(num.parse().unwrap(), rule);
        map
    })
}

fn parse_messages(s: &str) -> Vec<String> {
    s.lines().map(|s| s.to_string()).collect()
}

fn parse_file() -> (Rules, Vec<String>) {
    let input = read_to_string("data/day19.txt").unwrap();
    let (rules, messages) = input.split_once("\n\n").unwrap();
    (parse_rules(rules), parse_messages(messages))
}
