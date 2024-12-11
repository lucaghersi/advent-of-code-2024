use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "125 17";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str, repeat: u8) -> Result<u64> {
        let mut stones_count = 0;
        let mut memo: HashMap<(u64, u8), u64> = HashMap::new();
        for stone in parse(input).await {
            stones_count += blink_n_times(stone, repeat, &mut memo);
        }
        Ok(stones_count)
    }

    let result = part1(TEST, 6).await?;
    println!("Test Result 1.1 = {}", result);
    assert_eq!(22, result);
    let result = part1(TEST, 25).await?;
    println!("Test Result 1.2 = {}", result);
    assert_eq!(55312, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input, 25).await?);
    println!("Result 1 = {}", result);
    assert_eq!(184927, result);

    println!("=== Part 2 ===");

    let result = time_snippet!(part1(&input, 75).await?);
    println!("Result 2 = {}", result);
    assert_eq!(220357186726677, result);

    Ok(())
}

const MULTIPLY_BY: u64 = 2024;

async fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|f| f.parse::<u64>().unwrap())
        .collect()
}

fn blink_n_times(stone: u64, blinks: u8, memo: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(&cached) = memo.get(&(stone, blinks)) {
        return cached;
    }

    let mut count = 0;
    for stone in blink(stone) {
        count += blink_n_times(stone, blinks - 1, memo);
    }

    memo.insert((stone, blinks), count);
    count
}

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let digits = stone.checked_ilog10().unwrap_or(0) + 1;
    if digits % 2 == 0 {
        let divisor = 10_u64.pow(digits / 2);
        vec![stone / divisor, stone % divisor]
    } else {
        vec![stone * MULTIPLY_BY]
    }
}
