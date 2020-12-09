#![feature(str_split_once)]

use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        !(self.byr.is_empty()
            || self.iyr.is_empty()
            || self.eyr.is_empty()
            || self.hgt.is_empty()
            || self.hcl.is_empty()
            || self.ecl.is_empty()
            || self.pid.is_empty())
    }

    fn is_valid_part2(&self) -> bool {
        (Some(1920)..=Some(2002)).contains(&self.byr.parse::<u32>().ok())
            && (Some(2010)..=Some(2020)).contains(&self.iyr.parse::<u32>().ok())
            && (Some(2020)..=Some(2030)).contains(&self.eyr.parse::<u32>().ok())
            && (Some(2020)..=Some(2030)).contains(&self.eyr.parse::<u32>().ok())
            && ((Some(150)..=Some(193)).contains(
                &self
                    .hgt
                    .strip_suffix("cm")
                    .and_then(|hgt| hgt.parse::<u32>().ok()),
            ) || (Some(59)..=Some(76)).contains(
                &self
                    .hgt
                    .strip_suffix("in")
                    .and_then(|hgt| hgt.parse::<u32>().ok()),
            ))
            && self
                .hcl
                .strip_prefix('#')
                .map(|hcl| hcl.chars().all(|c| c.is_ascii_hexdigit()))
                .unwrap_or(false)
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_str())
            && (self.pid.chars().count() == 9 && self.pid.chars().all(|c| c.is_ascii_digit()))
    }
}

#[derive(Debug, Default)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn new(s: &str) -> Self {
        let mut passport = Passport::default();
        for f in s.split(&['\n', ' '][..]) {
            let (k, v) = f.split_once(':').unwrap();
            match k {
                "byr" => passport.byr = v.to_string(),
                "iyr" => passport.iyr = v.to_string(),
                "eyr" => passport.eyr = v.to_string(),
                "hgt" => passport.hgt = v.to_string(),
                "hcl" => passport.hcl = v.to_string(),
                "ecl" => passport.ecl = v.to_string(),
                "pid" => passport.pid = v.to_string(),
                "cid" => passport.cid = v.to_string(),
                k => panic!("unknown key {}", k),
            }
        }

        passport
    }
}

fn part1() -> usize {
    let passports = parse_file();
    passports.iter().filter(|p| p.is_valid_part1()).count()
}

fn part2() -> usize {
    let passports = parse_file();
    passports.iter().filter(|p| p.is_valid_part2()).count()
}

fn parse_file() -> Vec<Passport> {
    read_to_string("data/day04.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| Passport::new(s))
        .collect()
}
