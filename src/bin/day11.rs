#![feature(iter_map_while)]

use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let mut ferry = parse_file();
    while let Some(f) = run(&ferry, 4, 1) {
        ferry = f;
    }
    ferry.occupancy()
}

fn part2() -> usize {
    let mut ferry = parse_file();
    while let Some(f) = run(&ferry, 5, i32::MAX) {
        ferry = f;
    }
    ferry.occupancy()
}

fn run(ferry: &Ferry, threshold: usize, range: i32) -> Option<Ferry> {
    let occupants = ferry.occupancy();
    let new_layout = (0..ferry.0.len())
        .map(|y| {
            (0..ferry.0[0].len())
                .map(|x| ferry.new_seat(x, y, threshold, range))
                .collect()
        })
        .collect();
    let new_ferry = Ferry(new_layout);
    if new_ferry.occupancy() != occupants {
        Some(new_ferry)
    } else {
        None
    }
}

#[rustfmt::skip]
const ADJACENT: &[(i32, i32)] = &[(-1, -1), (-1, 0), (-1, 1), 
                                   (0, -1),           (0, 1), 
                                   (1, -1),  (1, 0),  (1, 1)];

#[derive(Debug)]
struct Ferry(Vec<Vec<char>>);

impl Ferry {
    fn occupancy(&self) -> usize {
        self.0.iter().flatten().filter(|seat| seat == &&'#').count()
    }

    fn adjacent(&self, x: usize, y: usize, range: i32) -> usize {
        ADJACENT
            .iter()
            .filter(|(dx, dy)| {
                (1..=range)
                    .map_while(|mul| {
                        self.0
                            .get((dy * mul + y as i32) as usize)
                            .and_then(|row| row.get((dx * mul + x as i32) as usize))
                    })
                    .find(|seat| seat == &&'#' || seat == &&'L')
                    == Some(&'#')
            })
            .count()
    }

    fn new_seat(&self, x: usize, y: usize, threshold: usize, range: i32) -> char {
        match self.0[y][x] {
            'L' if self.adjacent(x, y, range) == 0 => '#',
            '#' if self.adjacent(x, y, range) >= threshold => 'L',
            seat => seat,
        }
    }
}

fn parse_file() -> Ferry {
    let layout = read_to_string("data/day11.txt")
        .unwrap()
        .lines()
        .map(|row| row.chars().collect())
        .collect();
    Ferry(layout)
}
