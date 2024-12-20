use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use image::{ImageBuffer, Rgb, RgbImage};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use tokio::task::JoinSet;

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

const TEST2: &str = "\
r, wr, b, g, bwu, rb, gb, br

bbrgwb

";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<usize> {
        let mut onsen = Onsen::parse(input);
        onsen.calculate_possible_designs();
        Ok(onsen.valid_designs.len())
    }

    // async fn part2(input: &str, max_width: usize, max_height: usize) -> Result<usize> {
    //     let mut result = parse(input).await;
    //     let total = look_for_easter_egg(&mut result, max_width, max_height).await;
    //     Ok(total)
    // }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(6, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);
    assert_eq!(231, result);
    //
    // println!("=== Part 2 ===");
    //
    // part2(&input, MAX_WIDTH, MAX_HEIGHT).await?;
    // 6398

    Ok(())
}

struct Onsen {
    stripes: Vec<String>,
    required_designs: Vec<String>,
    valid_designs: HashSet<String>,
    stripes_max_length: usize,
}

impl Onsen {
    fn parse(input: &str) -> Self {
        let input = input.lines().collect_vec();

        let stripes = input[0]
            .split(',')
            .map(|v| v.trim().to_string())
            .collect_vec();
        let stripes_max_length = stripes
            .as_slice()
            .iter()
            .max_by(|x, y| x.len().cmp(&y.len()))
            .unwrap()
            .len();

        Onsen {
            stripes,
            required_designs: input
                .as_slice()
                .split_at(1)
                .1
                .iter()
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .collect_vec(),
            valid_designs: HashSet::new(),
            stripes_max_length,
        }
    }

    fn calculate_possible_designs(&mut self) {
        for rd in self.required_designs.as_slice() {
            let rdc = rd.chars().collect_vec();
            let mut r_pdc: Vec<char> = Vec::new();
            let mut combo_size: usize = self.stripes_max_length;
            let mut combo_position: usize = 0;

            while combo_position < rdc.len() {
                let stripe_to_check: String =
                    String::from_iter(&rdc[combo_position..combo_position + combo_size]);
                if self.stripes.contains(&stripe_to_check) {
                    let mut chars_to_append = stripe_to_check.chars().collect_vec();
                    r_pdc.append(&mut chars_to_append);
                    combo_position = r_pdc.len();
                    combo_size = rdc.len() - combo_position;
                    continue;
                } else {
                    combo_size -= 1;

                    if combo_size == 0 {
                        break;
                    }
                }
            }

            if rdc.len() == r_pdc.len() && rdc.iter().all(|c| r_pdc.iter().contains(c)) {
                self.valid_designs.insert(rd.clone());
            }
        }

        for rd in self.required_designs.as_slice() {
            let rdc = rd.chars().collect_vec();
            let mut r_pdc: Vec<char> = Vec::new();
            let mut combo_size: usize = 1;
            let mut combo_position: usize = 0;

            while combo_position < rdc.len() {
                let stripe_to_check: String =
                    String::from_iter(&rdc[combo_position..combo_position + combo_size]);
                if self.stripes.contains(&stripe_to_check) {
                    let mut chars_to_append = stripe_to_check.chars().collect_vec();
                    r_pdc.append(&mut chars_to_append);
                    combo_position = r_pdc.len();
                    combo_size = 1;
                    continue;
                } else {
                    combo_size += 1;

                    if combo_size + combo_position >= rdc.len() {
                        break;
                    }
                }
            }

            if rdc.len() == r_pdc.len() && rdc.iter().all(|c| r_pdc.iter().contains(c)) {
                self.valid_designs.insert(rd.clone());
            }
        }
    }
}
