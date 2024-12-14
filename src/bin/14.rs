use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use image::{ImageBuffer, Rgb, RgbImage};
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use tokio::task::JoinSet;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

const TEST2: &str = "\
p=2,4 v=2,-3
";

const MAX_HEIGHT: usize = 103;
const MAX_WIDTH: usize = 101;
const MAX_SECONDS: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str, max_width: usize, max_height: usize) -> Result<usize> {
        let result = parse(input).await;
        let total = execute_move(result, max_width, max_height).await;
        Ok(total)
    }

    async fn part2(input: &str, max_width: usize, max_height: usize) -> Result<usize> {
        let mut result = parse(input).await;
        let total = look_for_easter_egg(&mut result, max_width, max_height).await;
        Ok(total)
    }

    let result = part1(TEST, 11, 7).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(12, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input, MAX_WIDTH, MAX_HEIGHT).await?);
    println!("Result 1 = {}", result);
    assert_eq!(219512160, result);

    println!("=== Part 2 ===");

    part2(&input, MAX_WIDTH, MAX_HEIGHT).await?;
    // 6398

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Bot {
    position_x: usize,
    position_y: usize,
    velocity_x: isize,
    velocity_y: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

async fn parse(input: &str) -> Vec<Bot> {
    let mut bots: Vec<Bot> = Vec::new();
    let rule = Regex::new("p=(?<px>\\d*),(?<py>\\d*) v=(?<vx>-?\\d*),(?<vy>-?\\d*)").unwrap();
    let captures = rule.captures_iter(input).collect_vec();

    for capture in captures.iter() {
        bots.push(Bot {
            position_x: capture
                .name("px")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            position_y: capture
                .name("py")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            velocity_x: capture
                .name("vx")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
            velocity_y: capture
                .name("vy")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap(),
        })
    }

    bots
}

async fn look_for_easter_egg(bots: &mut [Bot], max_width: usize, max_height: usize) -> usize {
    let mut i = 0;

    loop {
        i += 1;

        for bot in bots.iter_mut() {
            let mut pos_x = bot.position_x as isize;
            let mut pos_y = bot.position_y as isize;
            make_one_move(
                bot,
                &mut pos_x,
                &mut pos_y,
                max_width as isize,
                max_height as isize,
            );
            bot.position_x = pos_x as usize;
            bot.position_y = pos_y as usize;
        }

        let mut buffer: RgbImage = ImageBuffer::new(MAX_WIDTH as u32, MAX_HEIGHT as u32);
        for x in 0..MAX_WIDTH {
            for y in 0..MAX_HEIGHT {
                let b = bots.iter().find(|b| b.position_x == x && b.position_y == y);
                if b.is_some() {
                    buffer.put_pixel(x as u32, y as u32, Rgb([255, 0, 0]));
                } else {
                    buffer.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
                }
            }
        }

        buffer
            .save(format!("images/image{}.png", i))
            .expect("Something is wrong");

        if i >= 10000 {
            break;
        }
    }

    i
}

async fn execute_move(bots: Vec<Bot>, max_width: usize, max_height: usize) -> usize {
    let mut set = JoinSet::new();
    for bot in bots {
        set.spawn(move_bot(bot, max_width as isize, max_height as isize));
    }
    let output = set.join_all().await;

    let mut total = 1;
    for q in output
        .iter()
        .filter_map(|p| get_quadrant(max_width, max_height, p))
        .sorted()
        .chunk_by(|&q| q)
        .into_iter()
    {
        total *= q.1.count();
    }

    total
}

fn get_quadrant(max_width: usize, max_height: usize, position: &Position) -> Option<usize> {
    let split_x;
    let split_y;
    let split_x2;
    let split_y2;

    if max_width % 2 == 0 {
        split_x = max_width / 2;
        split_x2 = split_x;
    } else {
        split_x = (max_width - 1) / 2;
        split_x2 = split_x + 1;
    };

    if max_height % 2 == 0 {
        split_y = max_height / 2;
        split_y2 = split_y;
    } else {
        split_y = (max_height - 1) / 2;
        split_y2 = split_y + 1;
    };

    if position.x < split_x {
        if position.y < split_y {
            return Some(1);
        } else if position.y >= split_y2 {
            return Some(3);
        }
    } else if position.x >= split_x2 {
        if position.y < split_y {
            return Some(2);
        } else if position.y >= split_y2 {
            return Some(4);
        }
    }

    None
}

async fn move_bot(bot: Bot, max_width: isize, max_height: isize) -> Position {
    let mut pos_x = bot.position_x as isize;
    let mut pos_y = bot.position_y as isize;

    for _ in 0..MAX_SECONDS {
        make_one_move(&bot, &mut pos_x, &mut pos_y, max_width, max_height);
    }

    Position {
        x: pos_x as usize,
        y: pos_y as usize,
    }
}

fn make_one_move(
    bot: &Bot,
    pos_x: &mut isize,
    pos_y: &mut isize,
    max_width: isize,
    max_height: isize,
) {
    let potential_x = *pos_x + bot.velocity_x;
    if potential_x < 0 {
        *pos_x = max_width - potential_x.abs();
    } else if potential_x > (max_width - 1) {
        *pos_x = potential_x - max_width;
    } else {
        *pos_x = potential_x;
    }

    let potential_y = *pos_y + bot.velocity_y;
    if potential_y < 0 {
        *pos_y = max_height - potential_y.abs();
    } else if potential_y > (max_height - 1) {
        *pos_y = potential_y - max_height;
    } else {
        *pos_y = potential_y;
    }
}
