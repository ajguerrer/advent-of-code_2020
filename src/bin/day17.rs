#![feature(min_const_generics)]

use std::{collections::HashSet, convert::TryInto, fs::read_to_string};

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let mut cube = parse_file::<3>();
    for _ in 0..6 {
        cube = cycle(&cube);
    }
    cube.len()
}

fn part2() -> usize {
    let mut cube = parse_file::<4>();
    for _ in 0..6 {
        cube = cycle(&cube);
    }
    cube.len()
}

fn cycle<const D: usize>(cube: &HashSet<Coord<D>>) -> HashSet<Coord<D>> {
    let neigh = cube
        .iter()
        .flat_map(|c| adjacent(c))
        .collect::<HashSet<_>>();
    neigh
        .iter()
        .filter(|n| {
            let active = cube.contains(*n);
            let adjacent = cube.intersection(&adjacent(n)).count();
            matches!((active, adjacent), (false, 3) | (true, 3) | (true, 4))
        })
        .cloned()
        .collect()
}

fn adjacent<const D: usize>(coord: &Coord<D>) -> HashSet<Coord<D>> {
    (0..3usize.pow(D as u32))
        .map(|i| {
            (0..D)
                .map(|c| coord[c] + (i / 3usize.pow(c as u32) % 3) as i32 - 1)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

type Coord<const D: usize> = [i32; D];

fn parse_line<const D: usize>((y, s): (usize, &str)) -> HashSet<Coord<D>> {
    s.chars()
        .enumerate()
        .filter_map(|(x, c)| match c {
            '#' => {
                let mut arr = [0; D];
                arr[0] = x as i32;
                arr[1] = y as i32;
                Some(arr)
            }
            '.' => None,
            c => panic!("weird input {}", c),
        })
        .collect()
}

fn parse_file<const D: usize>() -> HashSet<Coord<D>> {
    read_to_string("data/day17.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(parse_line)
        .collect()
}
