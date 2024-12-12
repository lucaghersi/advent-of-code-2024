use adv_code_2024::*;
use anyhow::*;
use async_recursion::async_recursion;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const TEST2: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

const TEST3: &str = "\
AAAAAA
ABBAAA
ABBAAA
AAABBA
AAABBA
AAAAAA
";

#[tokio::main]
async fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    async fn part1(input: &str) -> Result<usize> {
        let cost = walk(input, false);
        Ok(cost)
    }

    async fn part2(input: &str) -> Result<usize> {
        let cost = walk(input, true);
        Ok(cost)
    }

    let result = part1(TEST).await?;
    println!("Test Result 1 = {}", result);
    assert_eq!(1930, result);

    let mut input_file = File::open(INPUT_FILE)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer);

    let result = time_snippet!(part1(&input).await?);
    println!("Result 1 = {}", result);
    assert_eq!(1494342, result);

    println!("=== Part 2 ===");

    let result = part2(TEST).await?;
    println!("Test Result 2.1 = {}", result);
    assert_eq!(1206, result);
    let result = part2(TEST2).await?;
    println!("Test Result 2.2 = {}", result);
    assert_eq!(368, result);
    let result = part2(TEST3).await?;
    println!("Test Result 2.3 = {}", result);
    assert_eq!(368, result);

    let result = time_snippet!(part2(&input).await?);
    println!("Result 2 = {}", result);
    assert_eq!(893676, result);

    Ok(())
}

struct Map {
    map: Vec<Vec<char>>,
    map_height: usize,
    map_width: usize,
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    v: char,
}

#[derive(Hash, Eq, PartialEq)]
struct Corners {
    x: usize,
    y: usize,
}

struct Plot {
    value: char,
    area: usize,
    perimeter: usize,
    corners: usize,
}

fn parse(input: &str) -> Map {
    let mut map: Map = Map {
        map: Vec::new(),
        map_width: 0,
        map_height: 0,
    };

    for l in input.lines() {
        if l.is_empty() {
            break;
        }

        let mut inner: Vec<char> = Vec::new();
        for c in l.chars() {
            inner.push(c)
        }

        map.map.push(inner);
    }

    map.map_height = map.map.len();
    map.map_width = map.map[0].len();

    map
}

fn walk(input: &str, return_corners: bool) -> usize {
    let mut plots: Vec<Plot> = Vec::new();
    let mut points: HashSet<Point> = HashSet::new();
    let map = parse(input);

    for x in 0..map.map_height {
        for y in 0..map.map_width {
            let plant = map.map[x][y];

            if points.contains(&Point { x, y, v: plant }) {
                continue;
            }

            let mut corners: HashSet<Corners> = HashSet::new();
            let mut plot = Plot {
                value: plant,
                area: 0,
                perimeter: 0,
                corners: 0,
            };
            explore_plot(&map, &mut points, &mut corners, &mut plot, x, y);

            plots.push(plot);
        }
    }
    
    if return_corners {
        plots.iter().map(|p| p.area * p.corners).sum()
    } else {
        plots.iter().map(|p| p.area * p.perimeter).sum()
    }
}

fn explore_plot(
    map: &Map,
    points: &mut HashSet<Point>,
    corners: &mut HashSet<Corners>,
    plot: &mut Plot,
    x: usize,
    y: usize,
) {
    let plant = map.map[x][y];
    if plant == plot.value {
        if !(points.insert(Point {
            x,
            y,
            v: plot.value,
        })) {
            return;
        }
        plot.area += 1;
        plot.corners += count_cell_corners(map, plot.value, x, y, corners);
    } else {
        plot.perimeter += 1;
        return;
    }

    // clockwise check
    if x > 0 {
        // top
        explore_plot(map, points, corners, plot, x - 1, y);
    } else {
        plot.perimeter += 1;
    }

    if y < (map.map_width - 1) {
        // right
        explore_plot(map, points, corners, plot, x, y + 1);
    } else {
        plot.perimeter += 1;
    }

    if x < (map.map_height - 1) {
        // bottom
        explore_plot(map, points, corners, plot, x + 1, y);
    } else {
        plot.perimeter += 1;
    }

    if y > 0 {
        // left
        explore_plot(map, points, corners, plot, x, y - 1);
    } else {
        plot.perimeter += 1;
    }
}

fn count_cell_corners(
    map: &Map,
    v: char,
    x: usize,
    y: usize,
    corners: &mut HashSet<Corners>,
) -> usize {
    let top = || x != 0 && map.map[x - 1][y] == v;
    let bottom = || x != map.map_height - 1 && map.map[x + 1][y] == v;
    let left = || y != 0 && map.map[x][y - 1] == v;
    let right = || y != map.map_width - 1 && map.map[x][y + 1] == v;

    let top_left = || x > 0 && y > 0 && map.map[x - 1][y - 1] == v;
    let top_right = || x > 0 && y < map.map_width - 1 && map.map[x - 1][y + 1] == v;
    let bottom_left = || x < map.map_height - 1 && y > 0 && map.map[x + 1][y - 1] == v;
    let bottom_right =
        || x < map.map_height - 1 && y < map.map_width - 1 && map.map[x + 1][y + 1] == v;

    let top_left_corner = || {
        !((top() && !top_left() && !left())
            || (!top() && !top_left() && left())
            || (top() && top_left() && left()))
    };

    let top_right_corner = || {
        !((top() && !right() && !top_right())
            || (!top() && right() && !top_right())
            || (top() && right() && top_right()))
    };

    let bottom_left_corner = || {
        !((left() && bottom() && bottom_left())
            || (left() && !bottom() && !bottom_left())
            || (!left() && bottom() && !bottom_left()))
    };

    let bottom_right_corner = || {
        !((right() && bottom() && bottom_right())
            || (right() && !bottom() && !bottom_right())
            || (!right() && bottom() && !bottom_right()))
    };

    /* the extra section covers the x at the center problem
    AAAAAA
    AAAAAA
    AAABBA
    AAABBA
    ABBAAA
    ABBAAA
    AAAAAA
     */

    let mut counter = 0;
    if top_left_corner() && (corners.insert(Corners { x, y }) || (!top() && !left() && top_left()))
    {
        counter += 1;
    }
    if top_right_corner()
        && (corners.insert(Corners { x, y: y + 1 }) || (!top() && !right() && top_right()))
    {
        counter += 1;
    }
    if bottom_right_corner()
        && (corners.insert(Corners { x: x + 1, y: y + 1 })
            || (!bottom() && !right() && bottom_right()))
    {
        counter += 1;
    }
    if bottom_left_corner()
        && (corners.insert(Corners { x: x + 1, y }) || (!bottom() && !left() && bottom_left()))
    {
        counter += 1;
    }

    counter
}
