#![feature(str_split_once)]

use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> u32 {
    let (rules, _, nearby_tickets) = parse_file();
    nearby_tickets
        .iter()
        .flatten()
        .filter(|i| is_valid(**i, &rules))
        .sum()
}

fn part2() -> u64 {
    let (rules, my_ticket, nearby_tickets) = parse_file();
    let valid = nearby_tickets
        .iter()
        .filter(|t| !t.iter().any(|i| is_valid(*i, &rules)))
        .collect::<Vec<_>>();
    let fields = rules
        .iter()
        .map(|r| (r.name.clone(), find_valid_indices(&r.ranges, &valid)))
        .collect::<Vec<_>>();
    let fields = (0..valid[0].len())
        .scan(Vec::new(), |found, _| {
            let (name, index) = find_unique(found, &fields);
            found.push(index);
            Some((name, index))
        })
        .collect::<Vec<_>>();
    fields
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, i)| my_ticket[*i] as u64)
        .product()
}

fn is_valid(i: u32, rules: &[Rule]) -> bool {
    !rules
        .iter()
        .any(|rule| rule.ranges.iter().any(|r| r.contains(&i)))
}

fn find_valid_indices(ranges: &[RangeInclusive<u32>], tickets: &[&Vec<u32>]) -> Vec<usize> {
    (0..tickets[0].len())
        .filter(|i| {
            tickets
                .iter()
                .all(|t| ranges.iter().any(|r| r.contains(&t[*i])))
        })
        .collect()
}

fn find_unique(found: &[usize], fields: &[(String, Vec<usize>)]) -> (String, usize) {
    fields
        .iter()
        .find_map(|(name, indices)| {
            let indices = indices
                .iter()
                .filter(|i| !found.contains(i))
                .collect::<Vec<_>>();
            if indices.len() == 1 {
                Some((name.clone(), *indices[0]))
            } else {
                None
            }
        })
        .unwrap()
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<u32>>,
}

fn parse_rule(s: &str) -> Rule {
    let (name, ranges) = s.split_once(": ").unwrap();
    let name = name.to_string();
    let ranges = ranges
        .split(" or ")
        .map(|rule| {
            let (lower, upper) = rule.split_once('-').unwrap();
            lower.parse().unwrap()..=upper.parse().unwrap()
        })
        .collect();

    Rule { name, ranges }
}

fn parse_ticket(s: &str) -> Vec<u32> {
    s.split(',').map(|i| i.parse().unwrap()).collect()
}

fn parse_file() -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
    let input = read_to_string("data/day16.txt").unwrap();
    let (rules, my_ticket, nearby_tickets) = {
        let mut s = input.split("\n\n");
        (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
    };

    let rules = rules.lines().map(parse_rule).collect();
    let my_ticket = parse_ticket(my_ticket.lines().nth(1).unwrap());
    let nearby_tickets = nearby_tickets.lines().skip(1).map(parse_ticket).collect();
    (rules, my_ticket, nearby_tickets)
}
