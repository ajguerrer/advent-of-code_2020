#![feature(min_const_generics)]

use std::{collections::HashSet, fs::read_to_string};

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
    let next = cube
        .iter()
        .flat_map(|c| adjacent(*c))
        .collect::<HashSet<_>>();
    next.iter()
        .copied()
        .filter(|n| {
            let active = cube.contains(n);
            let adj = adjacent(*n)
                .filter(|a| a != n)
                .filter(|a| cube.contains(a))
                .count();
            matches!((active, adj), (false, 3) | (true, 2) | (true, 3))
        })
        .collect()
}

fn adjacent<const D: usize>(coord: Coord<D>) -> impl Iterator<Item = Coord<D>> {
    (0..3usize.pow(D as u32)).map(move |i| {
        let mut adj = [0; D];
        for (d, v) in adj.iter_mut().enumerate().take(D) {
            *v = coord[d] + (i / 3usize.pow(d as u32) % 3) as i32 - 1;
        }
        adj
    })
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
