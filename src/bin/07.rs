use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{iproduct, Itertools};
use std::fs::File;
use std::io::Read;
use std::iter::successors;
use tokio::task::JoinSet;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

const TEST2: &str = "\
21037: 9 7 18 13
";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<usize> {
        let matrix = parse(input)?;
        let result = process_matrix(&matrix, false).await?;
        Ok(result)
    }

    async fn part2(input: &str) -> Result<usize> {
        let matrix = parse(input)?;
        let result = process_matrix(&matrix, true).await?;
        Ok(result)
    }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(3749, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);
    
    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);
    
    println!("=== Part 2 ===");
    
    let result = part2(TEST).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!(11387, result);
    
    let result = time_snippet!(part2(&input).await?);
    println!("Result 2 = {}", result);

    Ok(())
}

async fn process_matrix(matrix: &[Vec<usize>], enable_pipe: bool)
    -> Result<usize> {

    let mut set = JoinSet::new();
    for eq in matrix {
        set.spawn(process_row(eq.clone(), enable_pipe));
    }
    let output = set.join_all().await;

    let total = output.iter().sum();
    Ok(total)
}

async fn process_row(equation: Vec<usize>, enable_pipe: bool) -> usize {

    let size = equation.len() - 1; // first is the result
    let expected_result = equation[0];
    
    let calc = |x:usize, y:usize, op: char| -> usize {
        if op == '+' {
            x + y
        } else if op == '*' {
            x * y
        } else {
            format!("{}{}", x, y).parse().unwrap()
        }
    };
    
    let operators = if enable_pipe { "*+|".chars() } else { "*+".chars() };
    
    for symbols in (1..=size-1).map(|_| operators.clone()).multi_cartesian_product()  {
        // println!("Testing combination {:?}", symbols);
        let mut combination_result = calc(equation[1], equation[2], symbols[0]);
        
        for i in 1..symbols.len() {
            combination_result = calc(combination_result, equation[i + 2], symbols[i]);
        }

        // println!("Result is {:?}", combination_result);
        if combination_result == expected_result {
            return combination_result;
        }
    }

    0
}

fn parse(input: &str) -> Result<Vec<Vec<usize>> > {
    let mut result: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        if line.is_empty(){
            break;
        }

        let mut row:Vec<usize> = Vec::new();

        let parsed_row = line.split(':').collect_vec();
        row.push(parsed_row[0].parse()?);
        parsed_row[1].split(' ').filter(|&i| !i.is_empty()).for_each(|i| row.push(i.parse().unwrap()));

        result.push(row);
    }

    Ok(result)
}