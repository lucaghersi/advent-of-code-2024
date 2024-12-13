use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use nalgebra::{Matrix2, Matrix2x1};
use regex::Regex;
use std::fs::File;
use std::io::Read;

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize_x: usize,
    prize_y: usize,
}

const A_COST: usize = 3;
const B_COST: usize = 1;
const MAX_TRY: f64 = 100.0;
const ADD_FACTOR: f64 = 10000000000000.0;

fn parse(input: &str) -> Vec<ClawMachine> {
    let mut claw_machines: Vec<ClawMachine> = Vec::new();

    let xy = Regex::new(
        "Button A: X\\+(?<XA>\\d*), Y\\+(?<YA>\\d*)\\nButton B: X\\+(?<XB>\\d*), Y\\+(?<YB>\\d*)",
    )
    .unwrap();
    let xy_prize = Regex::new("X=(?<X>\\d*), Y=(?<Y>\\d*)").unwrap();

    let prize_captures = xy_prize.captures_iter(input).collect_vec();

    for (i, capture) in xy.captures_iter(input).enumerate() {
        let xa = capture.name("XA").unwrap();
        let ya = capture.name("YA").unwrap();
        let xb = capture.name("XB").unwrap();
        let yb = capture.name("YB").unwrap();
        let prize_x = prize_captures[i].name("X").unwrap();
        let prize_y = prize_captures[i].name("Y").unwrap();

        let claw = ClawMachine {
            button_a: Button {
                x: xa.as_str().parse().unwrap(),
                y: ya.as_str().parse().unwrap(),
            },
            button_b: Button {
                x: xb.as_str().parse().unwrap(),
                y: yb.as_str().parse().unwrap(),
            },
            prize_x: prize_x.as_str().parse().unwrap(),
            prize_y: prize_y.as_str().parse().unwrap(),
        };

        claw_machines.push(claw);
    }

    claw_machines
}

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<usize> {
        let claw_machines = parse(input);
        let total = claw_machines
            .iter()
            .map(|c| try_get_prize(c, true, false))
            .sum();
        Ok(total)
    }

    async fn part2(input: &str) -> Result<usize> {
        let claw_machines = parse(input);
        let total = claw_machines
            .iter()
            .map(|c| try_get_prize(c, false, true))
            .sum();
        Ok(total)
    }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(480, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);
    assert_eq!(30973, result);

    println!("=== Part 2 ===");

    let result = part2(TEST).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!(875318608908, result);

    let result = time_snippet!(part2(&input).await?);
    println!("Result 2 = {}", result);
    assert_eq!(95688837203288, result);

    Ok(())
}

fn try_get_prize(claw_machine: &ClawMachine, limit: bool, add_factor: bool) -> usize {
    let m = Matrix2::from_vec(vec![
        claw_machine.button_a.x as f64,
        claw_machine.button_a.y as f64,
        claw_machine.button_b.x as f64,
        claw_machine.button_b.y as f64,
    ]);
    let m_inverted = m.try_inverse().unwrap();

    let r = Matrix2x1::from_vec(vec![
        if add_factor {
            claw_machine.prize_x as f64 + ADD_FACTOR
        } else {
            claw_machine.prize_x as f64
        },
        if add_factor {
            claw_machine.prize_y as f64 + ADD_FACTOR
        } else {
            claw_machine.prize_y as f64
        },
    ]);

    let solution = m_inverted * r;
    if (limit && (solution[0] > MAX_TRY || solution[1] > MAX_TRY))
        || solution[0] < 0.0
        || solution[1] < 0.0
    {
        return 0;
    }

    let a_count = solution[0].round() as usize;
    let b_count = solution[1].round() as usize;

    if (solution[0] - a_count as f64).abs() > 0.001 {
        return 0;
    }
    if (solution[1] - b_count as f64).abs() > 0.001 {
        return 0;
    }

    a_count * A_COST + b_count * B_COST
}
