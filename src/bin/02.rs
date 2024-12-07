use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{all, Itertools};
use std::fs::File;
use std::io::Read;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
51 54 57 60 61 64 67 64
10 1 2 3 4
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
1 1 1 1 1
1 2 3 4 4
4 4 3 2 1
9 8 7 2 1
1 1 2 3 4
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<usize> {
        let parsing = parse(input)?;
        let total_safe = parsing
            .iter()
            .filter(|&r| check(r))
            .count();
        Ok(total_safe)
    }

    fn part2(input: &str) -> Result<usize> {
        let parsing = parse(input)?;
        let total_safe = parsing
            .iter()
            .filter(|&r| is_safe_full(r).unwrap().0)
            .count();
        Ok(total_safe)
    }

    let result = part1(&TEST)?;
    println!("Test Result 1 = {}", result);
    assert_eq!(2, result);

    let result = part2(&TEST)?;
    println!("Test Result 2 = {}", result);
    assert_eq!(9, result);

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

fn parse(input: &str) -> Result<Vec<Vec<i64>>> {
    let mut reports: Vec<Vec<i64>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let levels: Vec<i64> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        reports.push(levels);
    }

    Ok(reports)
}

fn is_safe_full(report: &Vec<i64>) -> Result<(bool, &Vec<i64>)> {

    let report_result = check(&report);
    if report_result {
        return Ok((true, report));
    }

    for index_external in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(index_external);

        let report_result = check(&new_report);
        if report_result {
            return Ok((true, report));
        }
    }

    //println!("No valid: {}", report.iter().map(|f| f.to_string()).collect::<Vec<_>>().iter().join(" "));
    Ok((false, report))
}

fn check(new_report: &Vec<i64>) -> bool {
    let mut direction = 0isize;
    for i in 0..new_report.len() - 1 {
        let compare_result = compare(new_report[i], new_report[i + 1], &mut direction).unwrap();
        if !compare_result {
            return false;
        }
    }

    true
}

fn compare(first: i64, second: i64, direction: &mut isize) -> Result<bool> {
    let diff = first.abs_diff(second);
    if diff < 1 || diff > 3 {
        return Ok(false);
    }

    if first > second {
        if *direction == 0isize {
            *direction = 1;
        }

        return Ok(*direction == 1isize);
    }

    if *direction == 0isize {
        *direction = -1;
    }

    Ok(*direction == -1isize)
}
