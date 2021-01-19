#![feature(str_split_once)]

use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    // println!("{}", part2());
}

fn part1() -> u64 {
    let tiles = parse_file();
    tiles
        .iter()
        .filter(|tile| is_corner(tile, &tiles))
        .map(|tile| tile.id)
        .product()
}

fn part2() -> usize {
    let tiles = parse_file();
    0
}

fn is_corner(t1: &Tile, tiles: &[Tile]) -> bool {
    tiles
        .iter()
        .filter(|t2| t1.id != t2.id && side_matches(t1, t2))
        .count()
        == 2
}

fn side_matches(t1: &Tile, t2: &Tile) -> bool {
    t1.borders
        .iter()
        .filter_map(|s1| {
            t2.borders
                .iter()
                .find(|s2| s1 == *s2 || *s1 == s2.chars().rev().collect::<String>())
        })
        .count()
        > 0
}

#[derive(Debug)]
struct Tile {
    id: u64,
    borders: Vec<String>,
}

fn parse_tile(s: &str) -> Tile {
    let (id, tile) = s.split_once('\n').unwrap();
    let id = id.trim_start_matches("Tile ").trim_end_matches(':');
    let top = tile.lines().next().unwrap().to_string();
    let bottom = tile.lines().last().unwrap().to_string();
    let left = tile.lines().map(|line| line.chars().next().unwrap());
    let right = tile.lines().map(|line| line.chars().last().unwrap());
    Tile {
        id: id.parse().unwrap(),
        borders: vec![top, bottom, left.collect(), right.collect()],
    }
}

fn parse_file() -> Vec<Tile> {
    read_to_string("data/day20.txt")
        .unwrap()
        .split("\n\n")
        .map(|tile| parse_tile(tile))
        .collect()
}
