#![feature(str_split_once)]
#![feature(iter_advance_by)]

use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> i64 {
    let mut instrs = parse_file();
    run(&mut instrs).1
}

fn part2() -> i64 {
    let instrs = parse_file();
    for i in 0..instrs.len() {
        match instrs[i].0 {
            Instr::Jmp(num) => {
                let mut instrs = instrs.clone();
                instrs[i].0 = Instr::Nop(num);
                let (ok, val) = run(&mut instrs);
                if ok {
                    return val;
                }
            }
            Instr::Nop(num) => {
                let mut instrs = instrs.clone();
                instrs[i].0 = Instr::Jmp(num);
                let (ok, val) = run(&mut instrs);
                if ok {
                    return val;
                }
            }
            _ => {}
        }
    }
    0
}

fn run(instrs: &mut [(Instr, bool)]) -> (bool, i64) {
    let len = instrs.len();
    let mut acc = 0;
    let mut i = 0;
    loop {
        let (instr, visited) = &mut instrs[i];

        if *visited {
            return (false, acc);
        }

        match instr {
            Instr::Acc(num) => acc += *num as i64,
            Instr::Jmp(num) => {
                if *num > 0 {
                    i += *num as usize - 1
                } else if (num.abs() as usize) < i {
                    i -= num.abs() as usize + 1
                } else {
                    i += len - num.abs() as usize
                }
            }
            Instr::Nop(_) => {}
        }

        *visited = true;
        if i + 1 == len {
            return (true, acc);
        }

        i = (i + 1) % len;
    }
}

fn to_instr(s: &str) -> Instr {
    let (inst, num) = s.split_once(' ').unwrap();
    let num = num.trim_start_matches('+');
    match inst {
        "acc" => Instr::Acc(num.parse::<i32>().unwrap()),
        "jmp" => Instr::Jmp(num.parse::<i32>().unwrap()),
        "nop" => Instr::Nop(num.parse::<i32>().unwrap()),
        _ => panic!("bad inst"),
    }
}

fn parse_file() -> Vec<(Instr, bool)> {
    read_to_string("data/day08.txt")
        .unwrap()
        .lines()
        .map(|line| (to_instr(line), false))
        .collect()
}
