use adv_code_2024::*;
use anyhow::*;
use async_recursion::async_recursion;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<u32> {
        let result = parse(input).await;
        let total = process_map(&result, false).await;
        Ok(total)
    }

    async fn part2(input: &str) -> Result<u32> {
        let result = parse(input).await;
        let total = process_map(&result, true).await;
        Ok(total)
    }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(36, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);
    assert_eq!(798, result);

    println!("=== Part 2 ===");

    let result = part2(TEST).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!(81, result);

    let result = time_snippet!(part2(&input).await?);
    println!("Result 2 = {}", result);
    assert_eq!(1816, result);

    Ok(())
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

async fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let steps = line
            .chars()
            .map(|f| f.to_digit(10).unwrap_or(11))
            .collect_vec();
        map.push(steps);
    }

    //debug(&map);

    map
}

async fn process_map(map: &[Vec<u32>], total_trails: bool) -> u32 {
    let map_width = map[0].len();
    let map_height = map.len();

    let mut total = 0;

    for (x, l) in map.iter().enumerate() {
        for (y, v) in l.iter().enumerate() {
            if *v != 0 {
                continue;
            }

            if total_trails {
                total += step_count(map, map_height, map_width, x, y).await;
            } else {
                let trails = step(map, map_height, map_width, x, y).await;
                let mut nine_points: HashMap<Point, u32> = HashMap::new();
                for t in trails {
                    nine_points.entry(t).or_insert(0);
                }

                total += nine_points.keys().count() as u32
            }
        }
    }

    total
}

#[async_recursion]
async fn step_count(
    map: &[Vec<u32>],
    map_height: usize,
    map_width: usize,
    x: usize,
    y: usize,
) -> u32 {
    let value = map[x][y];
    if value == 9 {
        return 1;
    }

    let diff_is_acceptable = |other: u32| -> bool { other > value && other.abs_diff(value) == 1 };

    let mut total_trails = 0;

    // clockwise check
    if x > 0 && diff_is_acceptable(map[x - 1][y]) {
        // top
        total_trails += step_count(map, map_height, map_width, x - 1, y).await;
    }
    if y < (map_width - 1) && diff_is_acceptable(map[x][y + 1]) {
        // right
        total_trails += step_count(map, map_height, map_width, x, y + 1).await;
    }
    if x < (map_height - 1) && diff_is_acceptable(map[x + 1][y]) {
        // bottom
        total_trails += step_count(map, map_height, map_width, x + 1, y).await;
    }
    if y > 0 && diff_is_acceptable(map[x][y - 1]) {
        // left
        total_trails += step_count(map, map_height, map_width, x, y - 1).await;
    }

    total_trails
}

#[async_recursion]
async fn step(
    map: &[Vec<u32>],
    map_height: usize,
    map_width: usize,
    x: usize,
    y: usize,
) -> Vec<Point> {
    let value = map[x][y];
    if value == 9 {
        return vec![Point { x, y }];
    }

    let diff_is_acceptable = |other: u32| -> bool { other > value && other.abs_diff(value) == 1 };

    let mut nine_points: Vec<Point> = Vec::new();

    // clockwise check
    if x > 0 && diff_is_acceptable(map[x - 1][y]) {
        // top
        let mut top_point = step(map, map_height, map_width, x - 1, y).await;
        nine_points.append(&mut top_point);
    }
    if y < (map_width - 1) && diff_is_acceptable(map[x][y + 1]) {
        // right
        let mut right_point = step(map, map_height, map_width, x, y + 1).await;
        nine_points.append(&mut right_point);
    }
    if x < (map_height - 1) && diff_is_acceptable(map[x + 1][y]) {
        // bottom
        let mut bottom_point = step(map, map_height, map_width, x + 1, y).await;
        nine_points.append(&mut bottom_point);
    }
    if y > 0 && diff_is_acceptable(map[x][y - 1]) {
        // left
        let mut left_point = step(map, map_height, map_width, x, y - 1).await;
        nine_points.append(&mut left_point);
    }

    nine_points
}

// fn debug(map: &Vec<Vec<u32>>) {
//     for l in map {
//         let line = l.iter().map(|i| i.to_string()).collect::<String>();
//         println!("{}", line)
//     }
// }
