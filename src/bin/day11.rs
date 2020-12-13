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
    let new_layout = (0..ferry.height)
        .map(|y| {
            (0..ferry.width)
                .map(|x| ferry.new_seat(x, y, threshold, range))
                .collect()
        })
        .collect();
    let new_ferry = Ferry::new(new_layout);
    if new_ferry.occupancy() != ferry.occupancy() {
        Some(new_ferry)
    } else {
        None
    }
}

#[rustfmt::skip]
const ADJACENT: &[(i32, i32)] = &[(-1, -1), (-1, 0), (-1, 1), 
                                   (0, -1),           (0, 1), 
                                   (1, -1),  (1, 0),  (1, 1)];

#[derive(Debug, PartialEq, Copy, Clone)]
enum Seat {
    Occupied,
    Empty,
    Floor,
}

#[derive(Debug)]
struct Ferry {
    layout: Vec<Vec<Seat>>,
    width: usize,
    height: usize,
}

impl Ferry {
    fn new(layout: Vec<Vec<Seat>>) -> Self {
        let height = layout.len();
        let width = layout[0].len();
        Ferry {
            layout,
            height,
            width,
        }
    }

    fn occupancy(&self) -> usize {
        self.layout
            .iter()
            .flatten()
            .filter(|seat| seat == &&Seat::Occupied)
            .count()
    }

    fn adjacent(&self, x: usize, y: usize, range: i32) -> usize {
        ADJACENT
            .iter()
            .filter(|(dx, dy)| {
                (1..=range)
                    .map_while(|mul| {
                        let x = (dx * mul + x as i32) as usize;
                        let y = (dy * mul + y as i32) as usize;
                        self.layout.get(y).and_then(|row| row.get(x))
                    })
                    .find(|seat| seat == &&Seat::Occupied || seat == &&Seat::Empty)
                    == Some(&Seat::Occupied)
            })
            .count()
    }

    fn new_seat(&self, x: usize, y: usize, threshold: usize, range: i32) -> Seat {
        match self.layout[y][x] {
            Seat::Empty if self.adjacent(x, y, range) == 0 => Seat::Occupied,
            Seat::Occupied if self.adjacent(x, y, range) >= threshold => Seat::Empty,
            seat => seat,
        }
    }
}

fn parse_seat(c: char) -> Seat {
    match c {
        'L' => Seat::Empty,
        '.' => Seat::Floor,
        c => panic!("what is this seat: {}?", c),
    }
}

fn parse_file() -> Ferry {
    let layout = read_to_string("data/day11.txt")
        .unwrap()
        .lines()
        .map(|row| row.chars().map(parse_seat).collect())
        .collect();
    Ferry::new(layout)
}
