use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::ops::BitXor;

const DAY: &str = "22";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1
10
100
2024
";

const TEST2: &str = "\
1
2
3
2024
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<u64> {
        produce_secrets(input)
    }

    fn part2(input: &str) -> Result<u64> {
        produce_last(input)
    }

    let result = part1(TEST)?;
    println!("Test Result 1 = {}", result);
    assert_eq!(37327623, result);

    let result = part2(TEST2)?;
    println!("Test Result 2 = {}", result);
    assert_eq!(23, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input)?);
    println!("Result 1 = {}", result);
    assert_eq!(16999668565, result);

    let result = time_snippet!(part2(&input)?);
    println!("Result 2 = {}", result);
    assert_eq!(1898, result);

    Ok(())
}

fn produce_secrets(input: &str) -> Result<u64> {
    let secrets = input
        .lines()
        .collect_vec()
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();

    let total = secrets
        .iter()
        .map(|&s| {
            let mut secret: u64 = s;
            for _ in 0..2000 {
                secret = prune(mix(secret, secret * 64));
                secret = prune(mix(secret, secret.div_euclid(32)));
                secret = prune(mix(secret, secret * 2048));
            }
            secret
        })
        .sum::<u64>();

    Ok(total)
}

fn produce_last(input: &str) -> Result<u64> {
    let secrets = input
        .lines()
        .collect_vec()
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();

    let mut map: HashMap<Price, u64> = HashMap::new();
    let mut seen: HashSet<(u64, Price)> = HashSet::new();

    for s in secrets {
        let mut last_digits: Vec<i32> = Vec::new();

        let mut secret: u64 = s;
        for _ in 0..2000 {
            secret = prune(mix(secret, secret * 64));
            secret = prune(mix(secret, secret.div_euclid(32)));
            secret = prune(mix(secret, secret * 2048));

            let last_digit = (secret % 10) as i32;
            last_digits.push(last_digit);
        }

        calculate(s, &last_digits, &mut map, &mut seen);
    }

    let max = map.values().max().unwrap();

    Ok(*max)
}

#[derive(Eq, PartialEq, Hash)]
struct Price {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

fn calculate(
    key: u64,
    ld: &[i32],
    map: &mut HashMap<Price, u64>,
    seen: &mut HashSet<(u64, Price)>,
) {
    for i in 0..ld.len() - 5 {
        let a = ld[i + 1] - ld[i];
        let b = ld[i + 2] - ld[i + 1];
        let c = ld[i + 3] - ld[i + 2];
        let d = ld[i + 4] - ld[i + 3];
        let p = ld[i + 4] as u64;

        if !seen.insert((key, Price { a, b, c, d })) {
            continue;
        }

        map.entry(Price { a, b, c, d })
            .and_modify(|f| *f += p)
            .or_insert(p);
    }
}

fn mix(secret: u64, given: u64) -> u64 {
    given.bitxor(secret)
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
