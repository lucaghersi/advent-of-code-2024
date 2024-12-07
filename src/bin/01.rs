use anyhow::*;
use std::fs::File;
use std::io::{Read};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<u64> {
        let parsing = parse(input)?;
        let test_result = distance(&parsing.0, &parsing.1)?;
        Ok(test_result)
    }

    fn part2(input: &str) -> Result<u64> {
        let parsing = parse(input)?;
        let test_result = similarity(&parsing.0, &parsing.1)?;
        Ok(test_result)
    }

    // TODO: Set the expected answer for the test input
    let result = part1(&TEST)?;
    println!("Test Result 1 = {}", result);
    assert_eq!(11, result);

    // TODO: Set the expected answer for the test input
    let result = part2(&TEST)?;
    println!("Test Result 2 = {}", result);
    assert_eq!(31, result);

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

fn parse(input: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    let mut first = vec![0, 0];
    let mut second = vec![0, 0];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let items: Vec<u64> = line.split(' ')
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<u64>().unwrap()).collect();
        first.push(items[0].clone());
        second.push(items[1].clone());
    }

    first.sort();
    second.sort();

    Ok((first, second))
}

fn distance(first: &Vec<u64>, second: &Vec<u64>) -> Result<u64> {

    let mut total = 0;
    for i in 0..first.len() {
        total = total + first[i].abs_diff(second[i]);
    }

    Ok(total)
}

fn similarity(first: &Vec<u64>, second: &Vec<u64>) -> Result<u64> {

    let mut total = 0;
    for i in 0..first.len() {
        let count = second.iter().filter(|&n| *n == first[i]).count();
        total = total + first[i]*count as u64;
    }

    Ok(total)
}