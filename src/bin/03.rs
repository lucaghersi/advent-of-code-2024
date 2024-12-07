use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{all, Itertools};
use std::fs::File;
use std::io::Read;
use regex::Regex;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))^don't()_mul(5,5)+mul(32,64]";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<i64> {
        let parsing = parse(input)?;
        Ok(parsing)
    }

    fn part2(input: &str) -> Result<i64> {
        let parsing = parse_with_do_and_dont(input)?;
        Ok(parsing)
    }

    let result = part1(&TEST)?;
    println!("Test Result 1 = {}", result);
    assert_eq!(161, result);

    let result = part2(&TEST2)?;
    println!("Test Result 2 = {}", result);
    assert_eq!(48, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input)?);
    println!("Result 1 = {}", result);

    let result = time_snippet!(part2(&input)?);
    println!("Result 2 = {}", result);

    Ok(())
}

fn parse(input: &str) -> Result<i64> {
    let regex = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)")?;
    let mut it = regex.captures_iter(input);

    let mut total = 0i64;
    while let Some(capture) = it.next() {
        let first= &capture["first"].parse::<i64>()?;
        let second= &capture["second"].parse::<i64>()?;
        total = total + (first*second);
    }

    Ok(total)
}

fn parse_with_do_and_dont(input: &str) -> Result<i64> {

    let mut total = 0i64;
    let do_strings: Vec<&str> = input.split("do()").collect();
    for do_item in do_strings {
        let dont_string: Vec<&str> = do_item.split("don't()").collect();
        let do_string_to_use = dont_string[0];
        let string_value = parse(do_string_to_use)?;
        total = total + string_value;
    }

    Ok(total)
}
