#![feature(str_split_once)]

use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Instr {
    Acc,
    Jmp,
    Nop,
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> i32 {
    let instrs = parse_file();
    run(&instrs).1
}

fn part2() -> i32 {
    let instrs = parse_file();
    for i in 0..instrs.len() {
        let mut instrs = instrs.clone();
        match instrs[i] {
            (Instr::Jmp, num) => instrs[i] = (Instr::Nop, num),
            (Instr::Nop, num) => instrs[i] = (Instr::Jmp, num),
            _ => {}
        }
        let (ok, val) = run(&instrs);
        if ok {
            return val;
        }
    }
    panic!("no solution found")
}

fn run(instrs: &[(Instr, i32)]) -> (bool, i32) {
    let mut visited = vec![false; instrs.len()];
    let mut acc = 0;
    let mut pc = 0;

    loop {
        if pc == instrs.len() {
            return (true, acc);
        } else if visited[pc] {
            return (false, acc);
        }
        let mut incr = 1;
        let (instr, num) = instrs[pc];
        match instr {
            Instr::Acc => acc += num,
            Instr::Jmp => incr = num,
            Instr::Nop => {}
        };
        visited[pc] = true;
        pc = (pc as i32 + incr) as usize;
    }
}

fn to_instr(s: &str) -> (Instr, i32) {
    let (instr, num) = s.split_once(' ').unwrap();
    let num = num.trim_start_matches('+').parse::<i32>().unwrap();
    let instr = match instr {
        "acc" => Instr::Acc,
        "jmp" => Instr::Jmp,
        "nop" => Instr::Nop,
        _ => panic!("bad inst"),
    };
    (instr, num)
}

fn parse_file() -> Vec<(Instr, i32)> {
    read_to_string("data/day08.txt")
        .unwrap()
        .lines()
        .map(|line| to_instr(line))
        .collect()
}
