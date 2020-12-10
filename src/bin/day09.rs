use std::fs::read_to_string;

fn main() {
    let part1 = part1();
    println!("{}", part1);
    println!("{}", part2(part1));
}

fn part1() -> u64 {
    let nums = parse_file();
    nums[(25..nums.len())
        .find(|i| !is_xmas(&nums[i - 25..*i], nums[*i]))
        .unwrap()]
}

fn is_xmas(nums: &[u64], sum: u64) -> bool {
    nums.iter()
        .enumerate()
        .any(|(i, n1)| nums[i..].iter().any(|n2| n1 + *n2 == sum))
}

fn part2(tgt: u64) -> u64 {
    let nums = parse_file();
    let (i, j, _) = (1..nums.len())
        .scan((0, nums[0]), |(i, sum), j| {
            *sum += nums[j];
            while *sum > tgt {
                *sum -= nums[*i];
                *i += 1;
            }
            Some((*i, j, *sum))
        })
        .find(|(_, _, sum)| *sum == tgt)
        .unwrap();
    nums[i..j].iter().min().unwrap() + nums[i..j].iter().max().unwrap()
}

fn parse_file() -> Vec<u64> {
    read_to_string("data/day09.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
