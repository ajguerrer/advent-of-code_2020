use std::fs::read_to_string;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let path = parse_file();
    arboreal_encounters(&path, 3, 1)
}

fn part2() -> usize {
    let path = parse_file();
    let p1 = arboreal_encounters(&path, 1, 1);
    let p2 = arboreal_encounters(&path, 3, 1);
    let p3 = arboreal_encounters(&path, 5, 1);
    let p4 = arboreal_encounters(&path, 7, 1);
    let p5 = arboreal_encounters(&path, 1, 2);
    p1 * p2 * p3 * p4 * p5
}

fn arboreal_encounters(path: &[String], right: usize, down: usize) -> usize {
    let (_, sum) = path
        .iter()
        .skip(down)
        .step_by(down)
        .fold((right, 0), |(pos, sum), line| {
            (
                pos + right,
                sum + usize::from(line.chars().nth(pos % line.len()) == Some('#')),
            )
        });
    sum
}

fn parse_file() -> Vec<String> {
    read_to_string("data/day03.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}
