pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

use anyhow::Result;

fn main() -> Result<()> {
    println!("day 1, part 1: {}", day1::day1_part1()?);
    println!("day 1, part 2: {}", day1::day1_part2()?);
    println!("day 2, part 1: {}", day2::day2_part1()?);
    println!("day 2, part 2: {}", day2::day2_part2()?);
    println!("day 3, part 1: {}", day3::day3_part1()?);
    println!("day 3, part 2: {}", day3::day3_part2()?);
    println!("day 4, part 1: {}", day4::day4_part1()?);
    println!("day 4, part 2: {}", day4::day4_part2()?);
    println!("day 5, part 1: {}", day5::day5_part1()?);
    println!("day 5, part 2: {}", day5::day5_part2()?);

    Ok(())
}
