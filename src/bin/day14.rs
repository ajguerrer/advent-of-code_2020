#![feature(str_split_once)]

use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let program = parse_file();
    let (map, _) = program
        .iter()
        .fold((HashMap::new(), ""), |(mut map, mask), data| match data {
            Data::Mask(m) => (map, m),
            Data::Value(addr, val) => {
                map.insert(addr, apply_mask(*val, mask));
                (map, mask)
            }
        });
    map.values().map(|k| *k as usize).sum()
}

fn apply_mask(val: u64, mask: &str) -> u64 {
    mask.chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, mask)| match mask {
            '1' => acc + (1 << i),
            '0' => acc,
            'X' => acc + (val & (1 << i)),
            mask => panic!("weird mask {}", mask),
        })
}

fn part2() -> usize {
    let program = parse_file();
    let (map, _) = program
        .iter()
        .fold((HashMap::new(), ""), |(mut map, mask), data| match data {
            Data::Mask(m) => (map, m),
            Data::Value(addr, val) => {
                for addr in get_addrs(&floating_addr(*addr, mask)) {
                    map.insert(addr, *val);
                }
                (map, mask)
            }
        });
    map.values().map(|k| *k as usize).sum()
}

fn floating_addr(addr: u64, mask: &str) -> String {
    mask.chars()
        .rev()
        .enumerate()
        .fold(String::new(), |acc, (i, mask)| match mask {
            '1' => ["1", &acc].concat(),
            '0' => [((addr >> i) & 1).to_string(), acc].concat(),
            'X' => ["X", &acc].concat(),
            mask => panic!("weird mask {}", mask),
        })
}

fn get_addrs(mask: &str) -> Vec<u64> {
    if mask.chars().rev().any(|mask| mask == 'X') {
        vec![
            get_addrs(&mask.replacen('X', "1", 1)),
            get_addrs(&mask.replacen('X', "0", 1)),
        ]
        .concat()
    } else {
        vec![u64::from_str_radix(mask, 2).unwrap()]
    }
}

#[derive(Debug, PartialEq)]
enum Data {
    Mask(String),
    Value(u64, u64),
}

fn parse_line(s: &str) -> Data {
    let (data, value) = s.split_once(" = ").unwrap();
    if data == "mask" {
        Data::Mask(value.to_string())
    } else {
        let address = data
            .trim_start_matches("mem[")
            .trim_end_matches(']')
            .parse()
            .unwrap();
        Data::Value(address, value.parse().unwrap())
    }
}

fn parse_file() -> Vec<Data> {
    read_to_string("data/day14.txt")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect()
}
