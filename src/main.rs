pub mod day1;
pub mod day2;
pub mod day3;

use anyhow::Result;

fn main() -> Result<()> {
    println!("day 1, part 1: {}", day1::day1_part1()?);
    println!("day 1, part 2: {}", day1::day1_part2()?);
    println!("day 2, part 1: {}", day2::day2_part1()?);
    println!("day 2, part 2: {}", day2::day2_part2()?);
    println!("day 3, part 1: {}", day3::day3_part1()?);
    println!("day 3, part 2: {}", day3::day3_part2()?);

    Ok(())
}
