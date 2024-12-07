use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{all, Itertools};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

const TEST2: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

97,13,75,29,47
75,97,47,61,53
61,13,29
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1(input: &str) -> Result<u32> {
        let parsing = parse(input)?;
        let result = filter_invalid_manuals(&parsing)?;
        let total = sum_middle_pages(&result.valid_manuals)?;
        Ok(total)
    }

    fn part2(input: &str) -> Result<u32> {
        let parsing = parse(input)?;
        let result = filter_invalid_manuals(&parsing)?;

        let mut total = 0;
        for mut invalid_manual in result.invalid_manuals {
            while !fix_manual_2(&parsing, &mut invalid_manual)? {
                
            }
            total += invalid_manual[invalid_manual.len() / 2]
        }
        
        Ok(total)
    }

    let result = part1(TEST)?;
    println!("Test Result 1 = {}", result);
    assert_eq!(143, result);
    
    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);
    
    let result = time_snippet!(part1(&input)?);
    println!("Result 1 = {}", result);
    
    println!("=== Part 2 ===");

    let result = part2(TEST)?;
    println!("Test Result 2 = {}", result);
    assert_eq!(123, result);
    
    let result = time_snippet!(part2(&input)?);
    println!("Result 2 = {}", result);

    Ok(())
}

struct Manuals {
    valid_manuals: Vec<Vec<u32>>,
    invalid_manuals: Vec<Vec<u32>>,
}

fn fix_manual_2(data: &Stuff, manual: &mut Vec<u32>) -> Result<bool> {
    
    for index in 0..manual.len() {
        let page = manual[index];

        for left_index in 0..index {
            let page_at_left = manual[left_index];
            if !is_valid_before(&data.rules, &page, &page_at_left) {
                manual.swap(index, left_index);
                return Ok(false);
            }
        }

        for right_index in (index + 1)..manual.len() - 1 {
            let page_at_right = manual[right_index];
            if !is_valid_after(&data.rules, &page, &page_at_right) {
                manual.swap(index, right_index);
                return Ok(false);
            }
        }
    }

    Ok(true)
}

fn fix_manual(data: &Stuff, manual: &Vec<u32>) -> Result<u32> {

    let mut manual_to_fix = manual.clone();
    let mut counter = 0;
    
    while !check_manual(data, &manual_to_fix)? {
        
        let mut main_page_index = 0;

        loop {
            if main_page_index > manual_to_fix.len() - 2 {
                break;
            }

            //println!("{:#?}", manual_to_fix);

            let mut page_index = main_page_index;
            let mut next_page_index = main_page_index + 1;

            let mut page = manual_to_fix[page_index];
            let mut page_at_right = manual_to_fix[next_page_index];

            loop {
                //println!("checking {} vs {}", page, page_at_right);
                if is_valid_after(&data.rules, &page, &page_at_right) {
                    //println!("ok having {} before {}", page, page_at_right);
                } else {
                    //println!("swapping {} with {}", page, page_at_right);
                    manual_to_fix.swap(page_index, next_page_index);
                }

                page_index += 1;
                next_page_index += 1;

                if next_page_index > manual.len() - 1 {
                    main_page_index += 1;
                    break;
                }

                page = manual[page_index];
                page_at_right = manual[next_page_index];
            }
        }

        counter += 1;
        if counter == 5 {
            println!("{:#?}", manual);
            println!("{:#?}", manual_to_fix);
            panic!("too many cicles");
        }
        //println!("{:#?}", manual_to_fix);
    }
    
    Ok(manual_to_fix[manual_to_fix.len() / 2])
}

fn filter_invalid_manuals(data: &Stuff) -> Result<Manuals> {
    let mut valid_manuals: Vec<Vec<u32>> = Vec::new();
    let mut invalid_manuals: Vec<Vec<u32>> = Vec::new();

    for manual in &data.manuals {
        if check_manual(data, manual)? {
            valid_manuals.push(manual.clone());
        } else {
            invalid_manuals.push(manual.clone());
        }
    }

    Ok(Manuals {
        valid_manuals,
        invalid_manuals,
    })
}

fn sum_middle_pages(manuals: &Vec<Vec<u32>>) -> Result<u32> {
    let mut total = 0;

    for manual in manuals {
        let middle_page = manual[manual.len() / 2];
        total += middle_page;
    }

    Ok(total)
}

fn check_manual(data: &Stuff, manual: &[u32]) -> Result<bool> {
    for index in 0..manual.len() {
        let page = manual[index];

        for left_index in 0..index {
            let page_at_left = manual[left_index];
            if !is_valid_before(&data.rules, &page, &page_at_left) {
                return Ok(false);
            }
        }

        for right_index in (index + 1)..manual.len() - 1 {
            let page_at_right = manual[right_index];
            if !is_valid_after(&data.rules, &page, &page_at_right) {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

fn is_valid_before(rules: &HashMap<u32, Vec<u32>>, page: &u32, page_before: &u32) -> bool {
    if let Some(rule) = rules.get(page) {
        if rule.contains(page_before) {
            return false;
        }
    }

    true
}

fn is_valid_after(rules: &HashMap<u32, Vec<u32>>, page: &u32, page_after: &u32) -> bool {
    if let Some(rule) = rules.get(page) {
        if rule.contains(page_after) {
            return true;
        }
    }

    if let Some(rule) = rules.get(page_after) {
        if rule.contains(page) {
            return false;
        }
    }

    true
}

struct Stuff {
    rules: HashMap<u32, Vec<u32>>,
    manuals: Vec<Vec<u32>>,
}

fn parse(input: &str) -> Result<Stuff> {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    let input_split: Vec<&str> = input.split("\n\n").filter(|&s| !s.is_empty()).collect();

    let rules_to_parse = input_split[0];
    let manuals_to_parse = input_split[1];

    // sort rules
    for rule in rules_to_parse.lines() {
        let rule_split: Vec<&str> = rule.split('|').collect();
        let first: u32 = rule_split[0].parse()?;
        let second: u32 = rule_split[1].parse()?;

        if let std::collections::hash_map::Entry::Vacant(e) = rules.entry(first) {
            e.insert(vec![second]);
        } else {
            let content = rules.get_mut(&first).unwrap();
            content.push(second);
            content.sort();
        }
    }

    let mut manuals: Vec<Vec<u32>> = Vec::new();
    for manual in manuals_to_parse.lines() {
        let pages: Vec<u32> = manual
            .split(',')
            .map(|p| p.parse::<u32>().unwrap())
            .collect();
        manuals.push(pages);
    }

    Ok(Stuff { rules, manuals })
}
