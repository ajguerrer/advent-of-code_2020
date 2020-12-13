#![feature(str_split_once)]

use std::{
    fs::read_to_string,
    ops::{AddAssign, SubAssign},
};

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> i64 {
    let instructions = parse_file();
    let mut ship = Ship::new();
    for i in instructions {
        ship.advance_part1(i);
    }
    ship.manhatten()
}

fn part2() -> i64 {
    let instructions = parse_file();
    let mut ship = Ship::new();
    for i in instructions {
        ship.advance_part2(i);
    }
    ship.manhatten()
}

#[derive(Debug)]
struct Ship {
    dir: Direction,
    x: i64,
    y: i64,
    way: Waypoint,
}

impl Ship {
    fn new() -> Self {
        Ship {
            dir: Direction::East,
            x: 0,
            y: 0,
            way: Waypoint { y: 1, x: 10 },
        }
    }

    fn advance_part1(&mut self, inst: (Action, i64)) {
        match inst {
            (Action::North, d) => self.y += d,
            (Action::South, d) => self.y -= d,
            (Action::East, d) => self.x += d,
            (Action::West, d) => self.x -= d,
            (Action::Left, d) => self.dir -= d,
            (Action::Right, d) => self.dir += d,
            (Action::Forward, d) => match self.dir {
                Direction::North => self.y += d,
                Direction::South => self.y -= d,
                Direction::East => self.x += d,
                Direction::West => self.x -= d,
            },
        }
    }

    fn advance_part2(&mut self, inst: (Action, i64)) {
        match inst {
            (Action::North, d) => self.way.y += d,
            (Action::South, d) => self.way.y -= d,
            (Action::East, d) => self.way.x += d,
            (Action::West, d) => self.way.x -= d,
            (Action::Left, d) => match d {
                180 => self.way.one_eighty(),
                90 => self.way.counter_ninety(),
                270 => self.way.clock_ninety(),
                d => panic!("cant rotate left by {}", d),
            },
            (Action::Right, d) => match d {
                180 => self.way.one_eighty(),
                90 => self.way.clock_ninety(),
                270 => self.way.counter_ninety(),
                d => panic!("cant rotate right by {}", d),
            },
            (Action::Forward, d) => {
                self.x += self.way.x * d;
                self.y += self.way.y * d;
            }
        }
    }

    fn manhatten(&self) -> i64 {
        println!("{} {}", self.x, self.y);
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Waypoint {
    y: i64,
    x: i64,
}

impl Waypoint {
    fn one_eighty(&mut self) {
        self.y = -self.y;
        self.x = -self.x;
    }

    fn clock_ninety(&mut self) {
        let x = self.x;
        self.x = self.y;
        self.y = -x;
    }

    fn counter_ninety(&mut self) {
        let x = self.x;
        self.x = -self.y;
        self.y = x;
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North = 0,
    East = 90,
    South = 180,
    West = 270,
}

impl From<i64> for Direction {
    fn from(i: i64) -> Self {
        match i {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            _ => panic!("invalid direction {}", i),
        }
    }
}

impl AddAssign<i64> for Direction {
    fn add_assign(&mut self, rhs: i64) {
        *self = Direction::from((*self as i64 + rhs) % 360)
    }
}

impl SubAssign<i64> for Direction {
    fn sub_assign(&mut self, rhs: i64) {
        match rhs {
            90 => *self += 270,
            270 => *self += 90,
            rhs => *self += rhs,
        }
    }
}
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

fn parse_line(s: &str) -> (Action, i64) {
    let (action, dist) = s.split_at(1);
    let action = match action {
        "N" => Action::North,
        "S" => Action::South,
        "E" => Action::East,
        "W" => Action::West,
        "L" => Action::Left,
        "R" => Action::Right,
        "F" => Action::Forward,
        a => panic!("what is this action: {}", a),
    };
    (action, dist.parse().unwrap())
}

fn parse_file() -> Vec<(Action, i64)> {
    read_to_string("data/day12.txt")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect()
}
