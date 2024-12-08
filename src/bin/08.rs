use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

trait CartesianOperations<T> {
    fn directional_add(&self, other: &T) -> T;
    fn directional_sub(&self, other: &T) -> T;
}

impl CartesianOperations<Point> for Point {
    fn directional_add(&self, other: &Point) -> Point {
        let abs_x_distance = self.x.abs_diff(other.x) as isize;
        let abs_y_distance = self.y.abs_diff(other.y) as isize;

        let mut result = Point {
            x: self.x + abs_x_distance,
            y: self.y,
        };

        match &self.y.cmp(&other.y) {
            Ordering::Greater => {
                result.y = self.y + abs_y_distance;
            }
            Ordering::Less => {
                result.y = self.y - abs_y_distance;
            }
            Ordering::Equal => (),
        }

        result
    }

    fn directional_sub(&self, other: &Point) -> Point {
        let abs_x_distance = self.x.abs_diff(other.x) as isize;
        let abs_y_distance = self.y.abs_diff(other.y) as isize;

        let mut result = Point {
            x: self.x - abs_x_distance,
            y: self.y,
        };

        match &other.y.cmp(&self.y) {
            Ordering::Greater => {
                result.y = self.y - abs_y_distance;
            }
            Ordering::Less => {
                result.y = self.y + abs_y_distance;
            }
            Ordering::Equal => (),
        }

        result
    }
}

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<usize> {
        let total_antinodes = calculate_antinodes(input, false).await;
        Ok(total_antinodes)
    }

    async fn part2(input: &str) -> Result<usize> {
        let total_antinodes = calculate_antinodes(input, true).await;
        Ok(total_antinodes)
    }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(14, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);

    println!("=== Part 2 ===");

    let result = part2(TEST).await?;
    println!("Test Result 2 = {}", result);
    assert_eq!(34, result);

    let result = time_snippet!(part2(&input).await?);
    println!("Result 2 = {}", result);

    Ok(())
}

async fn calculate_antinodes(input: &str, resonate: bool) -> usize {
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();
    let mut anti: Vec<Point> = Vec::new();

    let max_x = (input.lines().count() - 1) as isize;
    let max_y = (input.lines().next().unwrap().len() - 1) as isize;

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            let x = x as isize;
            let y = y as isize;

            if let Some(antennas) = map.get_mut(&c) {
                antennas.push(Point { x, y });
                let resulting_anti_nodes =
                    process_antenna(Point { x, y }, antennas, max_x, max_y, resonate).await;

                for node in resulting_anti_nodes {
                    if !anti.contains(&node) {
                        anti.push(node);
                    }
                }
            } else {
                map.insert(c, vec![Point { x, y }]);
            }
        }
    }

    // debug
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c != '.' {
                print!("{c}");
            } else if anti.iter().any(|&f| f.x == x as isize && f.y == y as isize) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }

    anti.len()
}

async fn process_antenna(
    new_antenna: Point,
    other_antennas: &[Point],
    max_x: isize,
    max_y: isize,
    resonate: bool,
) -> Vec<Point> {
    let mut antinodes: Vec<Point> = Vec::new();

    for other_antenna in other_antennas {
        if new_antenna == *other_antenna {
            continue;
        }

        let calculate_up = |new: Point, old: Point| -> Option<Point> {
            let point = new.directional_add(&old);
            if point.x >= 0 && point.y >= 0 && point.x <= max_x && point.y <= max_y {
                return Some(point);
            }

            None
        };

        let calculate_down = |new: Point, old: Point| -> Option<Point> {
            let point = old.directional_sub(&new);
            if point.x >= 0 && point.y >= 0 && point.x <= max_x && point.y <= max_y {
                return Some(point);
            }

            None
        };

        let mut new = new_antenna;
        let mut old = *other_antenna;

        loop {
            let point_result = calculate_up(new, old);
            if let Some(point) = point_result {
                antinodes.push(point);
                old = new;
                new = point;
            } else {
                break;
            }

            if !resonate {
                break;
            }
        }

        let mut new = new_antenna;
        let mut old = *other_antenna;

        loop {
            let point_result = calculate_down(new, old);
            if let Some(point) = point_result {
                antinodes.push(point);
                new = old;
                old = point;
            } else {
                break;
            }

            if !resonate {
                break;
            }
        }

        if resonate {
            antinodes.push(new_antenna);
            antinodes.push(*other_antenna)
        }
    }

    antinodes
}
