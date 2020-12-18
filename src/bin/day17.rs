use std::{collections::HashSet, fs::read_to_string};

fn main() {
    // println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let mut cube = parse_file();
    for _ in 0..6 {
        cube = cycle(&cube);
    }
    cube.len()
}

fn part2() -> usize {
    let mut cube = parse_file();
    for _ in 0..6 {
        cube = cycle(&cube);
    }
    cube.len()
}

fn cycle(cube: &HashSet<Coord>) -> HashSet<Coord> {
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

fn adjacent(coord: &Coord) -> HashSet<Coord> {
    (coord[0] - 1..=coord[0] + 1)
        .flat_map(|x| {
            (coord[1] - 1..=coord[1] + 1).flat_map(move |y| {
                (coord[2] - 1..=coord[2] + 1)
                    .flat_map(move |z| (coord[3] - 1..=coord[3] + 1).map(move |w| [x, y, z, w]))
            })
        })
        .collect()
}

type Coord = [i32; 4];

fn parse_line((y, s): (usize, &str)) -> HashSet<Coord> {
    s.chars()
        .enumerate()
        .filter_map(|(x, c)| match c {
            '#' => Some([x as i32, y as i32, 0, 0]),
            '.' => None,
            c => panic!("weird input {}", c),
        })
        .collect()
}

fn parse_file() -> HashSet<Coord> {
    read_to_string("data/day17.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(parse_line)
        .collect()
}
