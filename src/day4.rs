use anyhow::{anyhow, bail, Result};
use std::{fs, str::FromStr};

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
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        if let Ok(byr) = self.byr.parse::<u32>() {
            byr >= 1920 && byr <= 2002
        } else {
            false
        }
    }

    fn is_iyr_valid(&self) -> bool {
        if let Ok(iyr) = self.iyr.parse::<u32>() {
            iyr >= 2010 && iyr <= 2020
        } else {
            false
        }
    }

    fn is_eyr_valid(&self) -> bool {
        if let Ok(eyr) = self.eyr.parse::<u32>() {
            eyr >= 2020 && eyr <= 2030
        } else {
            false
        }
    }

    fn is_hgt_valid(&self) -> bool {
        if let Some(hgt) = self.hgt.strip_suffix("cm") {
            if let Ok(hgt) = hgt.parse::<u32>() {
                hgt >= 150 && hgt <= 193
            } else {
                false
            }
        } else if let Some(hgt) = self.hgt.strip_suffix("in") {
            if let Ok(hgt) = hgt.parse::<u32>() {
                hgt >= 59 && hgt <= 76
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_hcl_valid(&self) -> bool {
        if let Some(hcl) = self.hcl.strip_prefix('#') {
            hcl.chars().all(|c| c.is_ascii_hexdigit())
        } else {
            false
        }
    }

    fn is_ecl_valid(&self) -> bool {
        self.ecl == "amb"
            || self.ecl == "blu"
            || self.ecl == "brn"
            || self.ecl == "gry"
            || self.ecl == "grn"
            || self.ecl == "hzl"
            || self.ecl == "oth"
    }

    fn is_pid_valid(&self) -> bool {
        self.pid.chars().count() == 9 && self.pid.chars().all(|c| c.is_ascii_digit())
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();
        for f in s.split(&['\n', ' '][..]) {
            let (k, v) = f.split_at(
                f.find(':')
                    .ok_or_else(|| anyhow!("could not find key/value pair"))?,
            );
            let v = &v[1..];
            match k {
                "byr" => passport.byr = v.to_string(),
                "iyr" => passport.iyr = v.to_string(),
                "eyr" => passport.eyr = v.to_string(),
                "hgt" => passport.hgt = v.to_string(),
                "hcl" => passport.hcl = v.to_string(),
                "ecl" => passport.ecl = v.to_string(),
                "pid" => passport.pid = v.to_string(),
                "cid" => passport.cid = v.to_string(),
                k => bail!("unknown key {}", k),
            }
        }

        Ok(passport)
    }
}

pub fn day4_part1() -> Result<String> {
    let passports = parse_file()?;
    let valid = passports.iter().filter(|p| p.is_valid_part1()).count();
    Ok(format!("{}/{} passports are valid", valid, passports.len()))
}

pub fn day4_part2() -> Result<String> {
    let passports = parse_file()?;
    let valid = passports.iter().filter(|p| p.is_valid_part2()).count();
    Ok(format!("{}/{} passports are valid", valid, passports.len()))
}

fn parse_file() -> Result<Vec<Passport>> {
    String::from_utf8_lossy(&fs::read("data/day4.txt")?)
        .split("\n\n")
        .map(|s| Ok(s.parse()?))
        .collect()
}
