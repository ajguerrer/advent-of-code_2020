use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let passes = parse_file();
    *passes.iter().max().unwrap()
}

fn part2() -> usize {
    let passes = parse_file();
    let max = passes.iter().max().unwrap();
    let front_seats = (1..*max).find(|p| passes.contains(p)).unwrap_or_default();
    (front_seats..*max)
        .find(|p| !passes.contains(p))
        .unwrap_or_default()
}

fn decode_pass(s: &str) -> usize {
    if s.chars().count() != 10 {
        panic!("pass must be 10 chars");
    }
    let (row, seat) = s.split_at(7);
    let row = row
        .chars()
        .rev()
        .enumerate()
        .fold(0, |row, (i, c)| match c {
            'B' => row + 2usize.pow(i as u32),
            'F' => row,
            _ => panic!("invalid row char"),
        });
    let seat = seat
        .chars()
        .rev()
        .enumerate()
        .fold(0, |seat, (i, c)| match c {
            'R' => seat + 2usize.pow(i as u32),
            'L' => seat,
            _ => panic!("invalid seat char"),
        });
    row * 8 + seat
}

fn parse_file() -> Vec<usize> {
    read_to_string("data/day05.txt")
        .unwrap()
        .lines()
        .map(|line| decode_pass(line))
        .collect()
}
