#![feature(str_split_once)]

use std::collections::HashMap;
use std::fs::read_to_string;

type BagMap = HashMap<String, Vec<(usize, String)>>;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let bags = parse_file();
    bags.keys()
        .map(|bag| contains("shiny gold", bag, &bags) as usize)
        .sum::<usize>()
        - 1
}

fn contains(name: &str, bag: &str, bags: &BagMap) -> bool {
    if bag == name {
        true
    } else {
        bags.get(bag)
            .unwrap()
            .iter()
            .any(|(_, bag)| contains(name, bag, bags))
    }
}

fn part2() -> usize {
    let bags = parse_file();
    count(1, "shiny gold", &bags) - 1
}

fn count(num: usize, bag: &str, bags: &BagMap) -> usize {
    let contents = bags.get(bag).unwrap();
    if contents.is_empty() {
        num
    } else {
        num + contents
            .iter()
            .map(|(num, bag)| count(*num, bag, &bags))
            .sum::<usize>()
            * num
    }
}

fn parse_bag(s: &str) -> (String, Vec<(usize, String)>) {
    let (name, contents) = s.split_once(" bags contain ").unwrap();
    let name = name.to_string();
    if contents.contains("no other bags") {
        return (name, Vec::new());
    }

    let contents = contents
        .split(", ")
        .map(|s| {
            let (count, name) = s.split_once(' ').unwrap();
            (
                count.parse::<usize>().unwrap(),
                name.trim_end_matches(&['s', '.'][..])
                    .trim_end_matches(" bag")
                    .to_string(),
            )
        })
        .collect();
    (name, contents)
}

fn parse_file() -> BagMap {
    let mut map = HashMap::new();
    for line in read_to_string("data/day07.txt").unwrap().lines() {
        let (k, v) = parse_bag(line);
        map.insert(k, v);
    }
    map
}
