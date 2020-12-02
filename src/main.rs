pub mod day1;

use anyhow::Result;
fn main() -> Result<()> {
    println!("day 1, part 1: {}", day1::day1_part1()?);
    println!("day 1, part 2: {}", day1::day1_part2()?);
    println!("day 1, part 1: {}", day1::day1_part1_easy_mode()?);
    println!("day 1, part 2: {}", day1::day1_part2_easy_mode()?);
    Ok(())
}
